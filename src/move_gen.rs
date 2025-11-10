use crate::board::Board;
use crate::coord::Square;
use crate::game::CastlingSide;
use crate::game::DrawKind;
use crate::game::FIFTY_MOVE_RULE_COUNT;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::GameState;
use crate::game::GameStateCore;
use crate::game::PieceCounts;
use crate::game::Position;
use crate::game::REPETITIONS_TO_FORCED_DRAW_COUNT;
use crate::game::RuleSet;
use crate::game::StepResult;
use crate::mov::KingMove;
use crate::mov::Move;
use crate::mov::MoveKind;
use crate::mov::PawnMove;
use crate::mov::Threat;
use crate::piece::Piece;
use crate::piece::PieceKind;
use crate::player::PlayerKind;
use alloc::vec;
use alloc::vec::Vec;
use core::ascii::Char as AsciiChar;
use core::ops::Not;

impl GameState {
    #[must_use]
    pub fn search(self, max_depth: u32, checker: impl Fn(&Self)) -> SearchStats {
        let mut terminated_games_checkmate: Vec<Self> = vec![];
        let mut terminated_games_draw: Vec<Self> = vec![];
        let mut continued_games: Vec<Self> = vec![self];
        let mut new_continued_games: Vec<Self> = vec![];

        let mut stats: SearchStats = SearchStats::default();

        for _ in 0..=max_depth {
            continued_games.clone().into_iter().for_each(|game| {
                checker(&game);
                let legal_moves: Vec<Move> = game.core.legal_moves().collect();

                for mov in legal_moves.clone() {
                    if matches!(mov.kind, MoveKind::Pawn(PawnMove::EnPassant { .. }),) {
                        stats.en_passant += 1;
                    }
                    match game.clone().step(mov, legal_moves.clone()) {
                        | StepResult::Terminated(GameResult {
                            kind: GameResultKind::Win,
                            final_game_state,
                        }) => terminated_games_checkmate.push(final_game_state),
                        | StepResult::Terminated(GameResult {
                            kind: GameResultKind::Draw(_),
                            final_game_state,
                        }) => terminated_games_draw.push(final_game_state),
                        | StepResult::Continued(game_state) => {
                            new_continued_games.push(game_state);
                        }
                    }
                }
            });

            core::mem::swap(&mut continued_games, &mut new_continued_games);
            new_continued_games.clear();
        }

        stats.continued_games = continued_games.len();
        stats.checkmated_games = terminated_games_checkmate.len();
        stats.drawn_games = terminated_games_draw.len();
        stats
    }

    #[cfg(feature = "rand")]
    pub fn random_walk(self, max_depth: u32, checker: impl Fn(&Self)) -> WalkStats {
        use crate::alloc::borrow::ToOwned as _;

        let mut rng = rand::rng();
        let mut game = self;

        let mut depth = 0;
        let mut move_history = vec![];

        while max_depth > depth {
            use crate::notation::san::long_algebraic_notation;
            use rand::seq::IndexedRandom;

            depth += 1;

            checker(&game);
            let legal_moves: Vec<Move> = game.core.legal_moves().collect();
            let random_move = legal_moves
                .choose(&mut rng)
                .expect("If the game has no legal moves, it should've ended last turn")
                .to_owned();

            move_history.push(long_algebraic_notation(game.clone(), random_move));

            match game.clone().step(random_move, legal_moves.clone()) {
                StepResult::Continued(game_state) => {
                    game = game_state;
                }
                StepResult::Terminated(GameResult {
                    final_game_state, ..
                }) => {
                    return WalkStats {
                        final_depth: depth,
                        final_game_state,
                        move_history,
                    };
                }
            }
        }
        WalkStats {
            final_depth: depth,
            final_game_state: game,
            move_history,
        }
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn step(mut self, mov: Move, all_legal_moves: Vec<Move>) -> StepResult {
        self.core.board.apply_move(mov);
        let mut game = self;

        let current_position = Position {
            board: game.core.board,
            possible_moves: all_legal_moves,
            castling_rights: game.core.castling_rights,
        };

        if game.rule_set != RuleSet::Perft {
            game.position_history.push(current_position.clone());
        }

        // handle fifty move rule
        if mov.kind.piece_kind() == PieceKind::Pawn || mov.is_capture() {
            game.core.fifty_move_rule_clock.reset();
        } else {
            game.core.fifty_move_rule_clock.increase();
        }

        // handle our own castling rights
        match mov.kind {
            MoveKind::King(_) => {
                game.core
                    .deny_castling(game.core.active_player, CastlingSide::Kingside);
                game.core
                    .deny_castling(game.core.active_player, CastlingSide::Queenside);
            }
            MoveKind::Rook { .. } => {
                for castling_side in [CastlingSide::Kingside, CastlingSide::Queenside] {
                    if mov.origin == game.core.active_player.rook_start(castling_side) {
                        game.core
                            .deny_castling(game.core.active_player, castling_side);
                    }
                }
            }
            _ => { /*nothing */ }
        }

        // handle opponents castling rights
        if mov.is_capture() && matches!(mov.kind, MoveKind::Pawn(PawnMove::EnPassant { .. })).not()
        {
            for castling_side in [CastlingSide::Kingside, CastlingSide::Queenside] {
                if mov.destination == game.core.active_player.opponent().rook_start(castling_side) {
                    game.core
                        .deny_castling(game.core.active_player.opponent(), castling_side);
                }
            }
        }

        // handle en passant target
        if mov.kind == MoveKind::Pawn(PawnMove::DoubleStep) {
            let en_passant_target = (mov.destination + game.core.active_player.backwards_one_row())
                .expect("this cannot be outside the board");
            game.core.en_passant_target = Some(en_passant_target);
        } else {
            game.core.en_passant_target = None;
        }

        let future = game.clone().with_opponent_active();
        if future.core.legal_moves().count() == 0 {
            return if future.core.board.is_king_checked(future.core.active_player) {
                StepResult::Terminated(GameResult {
                    kind: GameResultKind::Win,
                    final_game_state: game,
                })
            } else {
                StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::Stalemate),
                    final_game_state: game,
                })
            };
        }

        if game.rule_set != RuleSet::Perft {
            if game
                .position_history
                .iter()
                .filter(|&position| *position == current_position)
                .count()
                == REPETITIONS_TO_FORCED_DRAW_COUNT
            {
                return StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::ThreefoldRepetition),
                    final_game_state: game,
                });
            }

            if game.core.fifty_move_rule_clock == FIFTY_MOVE_RULE_COUNT {
                return StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::FiftyMove),
                    final_game_state: game,
                });
            }
        }

        let piece_counts = game.core.board.piece_counts();

        if piece_counts == PieceCounts::KINGS_ONLY {
            return StepResult::Terminated(GameResult {
                kind: GameResultKind::Draw(DrawKind::InsufficientMaterial),
                final_game_state: game,
            });
        }

        if game.core.active_player == PlayerKind::Black {
            game.core.full_move_count.increase();
        }
        game.core.active_player = game.core.active_player.opponent();
        StepResult::Continued(game)
    }
}

impl GameStateCore {
    pub fn legal_moves(&self) -> impl Iterator<Item = Move> {
        self.threatening_move_candidates()
            .chain(self.pawn_step_candidates())
            .chain(self.castle_candidates())
            .filter(move |mov| {
                self.board
                    .with_move_applied(*mov)
                    .is_king_checked(self.active_player)
                    .not()
            })
    }

    gen fn castle_candidates(&self) -> Move {
        for castling_side in CastlingSide::ALL {
            if self.has_castling_right(castling_side).not() {
                continue;
            }

            if self
                .are_castle_squares_free_from_checks_and_pieces(castling_side)
                .not()
            {
                continue;
            }

            yield Move {
                kind: MoveKind::King(KingMove::Castle {
                    rook_start: self.active_player.rook_start(castling_side),
                    rook_target: self.active_player.rook_castling_target(castling_side),
                    castling_side,
                }),
                origin: self.active_player.king_start(),
                destination: self.active_player.king_castling_target(castling_side),
            }
        }
    }

    fn threatening_move_candidates(&self) -> impl Iterator<Item = Move> {
        self.board
            .threatening_moves_by(self.active_player)
            .flat_map(|threat| self.threat_to_move_candidate(threat))
    }

    gen fn pawn_step_candidates(&self) -> Move {
        for square in Square::ALL {
            if self.board[square] != Some(PieceKind::Pawn.to_piece(self.active_player)) {
                continue;
            }

            let one_in_front = (square + self.active_player.forwards_one_row())
                .expect("a pawn cannot exist on the last row");

            if self.board[one_in_front].is_some() {
                continue; // pawns cant capture moving forward!
            }

            if one_in_front.row == self.active_player.pawn_promotion_row() {
                for promotion_option in PieceKind::PROMOTION_OPTIONS {
                    yield Move {
                        kind: MoveKind::Pawn(PawnMove::Promotion {
                            is_capture: false,
                            replacement: promotion_option.to_piece(self.active_player),
                        }),
                        origin: square,
                        destination: one_in_front,
                    }
                }
            } else {
                yield Move {
                    kind: MoveKind::Pawn(PawnMove::SimpleStep),
                    origin: square,
                    destination: one_in_front,
                }
            }

            let Ok(two_in_front) = square + self.active_player.forwards_one_row() * 2 else {
                continue; // this one can def be out of range.
            };

            if square.row != self.active_player.pawn_starting_row() {
                continue; // pawns can only double-move when they haven't moved yet!
            }

            if self.board[two_in_front].is_some() {
                continue; // pawns cant capture moving forward!
            }

            yield Move {
                kind: MoveKind::Pawn(PawnMove::DoubleStep),
                origin: square,
                destination: two_in_front,
            }
        }
    }

    #[must_use]
    fn threat_to_move_candidate(&self, threat: Threat) -> Vec<Move> {
        let is_capture = self.board[threat.destination].is_some();
        let origin = threat.origin;
        let destination = threat.destination;
        match threat.piece.kind {
            PieceKind::Knight => vec![Move {
                kind: MoveKind::Knight { is_capture },
                origin,
                destination,
            }],
            PieceKind::Bishop => vec![Move {
                kind: MoveKind::Bishop { is_capture },
                origin,
                destination,
            }],
            PieceKind::Rook => vec![Move {
                kind: MoveKind::Rook { is_capture },
                origin,
                destination,
            }],
            PieceKind::Queen => vec![Move {
                kind: MoveKind::Queen { is_capture },
                origin,
                destination,
            }],
            PieceKind::King => vec![Move {
                kind: MoveKind::King(KingMove::Normal { is_capture }),
                origin,
                destination,
            }],
            PieceKind::Pawn if is_capture => {
                if threat.destination.row == self.active_player.pawn_promotion_row() {
                    PieceKind::PROMOTION_OPTIONS
                        .iter()
                        .map(|promotion_option| Move {
                            kind: MoveKind::Pawn(PawnMove::Promotion {
                                is_capture: true,
                                replacement: promotion_option.to_piece(self.active_player),
                            }),
                            origin,
                            destination,
                        })
                        .collect()
                } else {
                    vec![Move {
                        kind: MoveKind::Pawn(PawnMove::SimpleCapture),
                        origin,
                        destination,
                    }]
                }
            }
            PieceKind::Pawn => {
                //en passant case, this is never gonna lead to promotion
                if Some(destination) == self.en_passant_target {
                    vec![Move {
                        kind: MoveKind::Pawn(PawnMove::EnPassant {
                            affected: (threat.destination + self.active_player.backwards_one_row())
                                .expect("how is this not on the board?"),
                        }),
                        origin,
                        destination,
                    }]
                } else {
                    vec![]
                }
            }
        }
    }
}

impl Board {
    pub const fn apply_move(&mut self, m: Move) {
        *self = self.with_move_applied(m);
    }

    #[must_use]
    pub const fn with_move_applied(mut self, m: Move) -> Self {
        self.mov(m.origin, m.destination);
        match m.kind {
            | MoveKind::Pawn(
                PawnMove::SimpleStep | PawnMove::DoubleStep | PawnMove::SimpleCapture,
            )
            | MoveKind::Knight { .. }
            | MoveKind::Bishop { .. }
            | MoveKind::Rook { .. }
            | MoveKind::Queen { .. }
            | MoveKind::King(KingMove::Normal { .. }) => { /*nothing */ }

            MoveKind::Pawn(PawnMove::EnPassant { affected }) => {
                self[affected] = None;
            }

            MoveKind::Pawn(PawnMove::Promotion { replacement, .. }) => {
                self[m.destination] = Some(replacement);
            }

            MoveKind::King(KingMove::Castle {
                rook_start,
                rook_target,
                ..
            }) => {
                self.mov(rook_start, rook_target);
            }
        }
        self
    }
}

#[derive(Debug, Default)]
pub struct SearchStats {
    pub continued_games: usize,
    pub checkmated_games: usize,
    pub drawn_games: usize,
    pub en_passant: usize,
}

#[derive(Debug, Default)]
pub struct WalkStats {
    pub final_depth: u32,
    pub final_game_state: GameState,
    pub move_history: Vec<Vec<AsciiChar>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameStateCore;
    use crate::notation::san::long_algebraic_notation;
    use crate::notation::san::standard_algebraic_notation;
    use crate::testing::skip_if_no_expensive_test_opt_in;
    use std::println;

    #[test]
    fn search() {
        skip_if_no_expensive_test_opt_in!();

        let depth = 3;
        let game = GameState::default();

        let before = std::time::Instant::now();

        let stats = game.search(depth, |_| ());

        println!("---------------------------");
        println!("depth: {depth}");
        println!("{stats:?}");
        println!("elapsed: {:?}", before.elapsed());
        println!("---------------------------");
    }

    #[test]
    fn perft() {
        skip_if_no_expensive_test_opt_in!();

        let depth = 3;
        let game = GameState::perft();

        let before = std::time::Instant::now();

        let stats = game.search(depth, |_| ());

        println!("---------------------------");
        println!("depth: {depth}");
        println!("{stats:?}");
        println!("elapsed: {:?}", before.elapsed());
        println!("---------------------------");
    }

    #[cfg(feature = "rand")]
    #[test]
    fn many_random_walks() {
        skip_if_no_expensive_test_opt_in!();

        let max_depth = 1_000;
        let walk_count = 100;
        let game = GameState::perft();

        for i in 0..walk_count {
            let stats = game.clone().random_walk(max_depth, owl_checker_depth_1);
            println!("{i}: {}", stats.final_depth);
        }
    }

    #[test]
    fn test_mass_fens() {
        skip_if_no_expensive_test_opt_in!();

        let fens = std::fs::read_to_string("./fens/fens.txt").unwrap();

        for fen in fens.lines() {
            let schach_game_core = GameStateCore::try_from_fen(fen).unwrap();
            let schach_fen = schach_game_core.to_fen();
            assert_eq!(fen, schach_fen.as_str());
        }
    }

    #[test]
    fn test_mass_against_owl() {
        skip_if_no_expensive_test_opt_in!();

        let max_depth = 2;
        let max_fens = 10;
        let skip_fens = 100;
        let progress_thingy = core::cmp::max(max_fens / 1_000, 1);
        let fens = std::fs::read_to_string("./fens/fens.txt").unwrap();

        let mut progress = 0;

        for fen in fens.lines().skip(skip_fens).take(max_fens) {
            let game_core = GameStateCore::try_from_fen(fen).unwrap();

            let game = GameState {
                core: game_core,
                position_history: vec![],
                rule_set: RuleSet::Standard,
            };

            let _ = game.search(max_depth, owl_checker_move_count);
            progress += 1;
            if progress % progress_thingy == 0 {
                println!("{progress}/{max_fens}");
            }
        }
    }

    fn owl_checker_move_count(game: &GameState) {
        let schach_move_count = game.core.legal_moves().count();
        let owl_move_count = owlchess::movegen::legal::gen_all(
            &owlchess::Board::from_fen(game.core.to_fen().as_str()).unwrap(),
        )
        .len();
        assert_eq!(schach_move_count, owl_move_count);
    }

    #[allow(dead_code)]
    fn owl_checker_depth_1(game: &GameState) {
        let schach_all_legals = game.core.legal_moves().collect::<Vec<_>>();
        for mov in &schach_all_legals {
            let schach_move_san = standard_algebraic_notation(game.clone(), *mov);
            let owl_board = owlchess::Board::from_fen(game.core.to_fen().as_str()).unwrap();
            let Ok(owl_move) = owlchess::Move::from_san(schach_move_san.as_str(), &owl_board)
            else {
                for mov in &schach_all_legals {
                    println!(
                        "lan: {}  san: {}",
                        long_algebraic_notation(game.clone(), *mov).as_str(),
                        standard_algebraic_notation(game.clone(), *mov).as_str()
                    );
                }
                panic!();
            };
            let new_owl_board = owl_board.make_move(owl_move).unwrap();
            let StepResult::Continued(new_schach_board) =
                game.clone().step(*mov, schach_all_legals.clone())
            else {
                continue;
            };

            let new_owl_move_count = owlchess::movegen::legal::gen_all(&new_owl_board)
                .iter()
                .count();

            let new_schach_move_count = new_schach_board.core.legal_moves().count();

            assert_eq!(new_schach_move_count, new_owl_move_count);
        }
    }
}
