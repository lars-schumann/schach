use crate::alloc::borrow::ToOwned;
use crate::coord::Square;
use crate::game::CastlingSide;
use crate::game::DrawKind;
use crate::game::FIFTY_MOVE_RULE_COUNT;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::GameState;
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
                let legal_moves: Vec<Move> = game.legal_moves().collect();

                for mov in legal_moves.clone() {
                    if matches!(
                        mov,
                        Move {
                            kind: MoveKind::Pawn(PawnMove::EnPassant { .. }),
                            ..
                        }
                    ) {
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
        let mut rng = rand::rng();
        let mut game = self;

        let mut depth = 0;
        let mut move_history = vec![];

        while max_depth > depth {
            use rand::seq::IndexedRandom;

            use crate::notation::algebraic::long_algebraic_notation;

            depth += 1;

            checker(&game);
            let legal_moves: Vec<Move> = game.legal_moves().collect();
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
    pub fn step(self, mov: Move, all_legal_moves: Vec<Move>) -> StepResult {
        let mut game = self.apply_move_to_board(mov);

        let current_position = Position {
            board: game.board,
            possible_moves: all_legal_moves,
            castling_rights: game.castling_rights,
        };

        if game.rule_set != RuleSet::Perft {
            game.position_history.push(current_position.clone());
        }

        // handle fifty move rule
        if mov.kind.piece_kind() == PieceKind::Pawn || mov.is_capture() {
            game.fifty_move_rule_clock.reset();
        } else {
            game.fifty_move_rule_clock.increase();
        }

        // handle our own castling rights
        match mov {
            Move {
                kind: MoveKind::King(_),
                ..
            } => {
                game.deny_castling(game.active_player, CastlingSide::Kingside);
                game.deny_castling(game.active_player, CastlingSide::Queenside);
            }
            Move {
                kind: MoveKind::Rook { .. },
                origin,
                ..
            } => {
                for castling_side in [CastlingSide::Kingside, CastlingSide::Queenside] {
                    if origin == game.active_player.rook_start(castling_side) {
                        game.deny_castling(game.active_player, castling_side);
                    }
                }
            }
            _ => { /*nothing */ }
        }

        // handle opponents castling rights
        if let Some(affected) = mov.capture_affected_square() {
            for castling_side in [CastlingSide::Kingside, CastlingSide::Queenside] {
                if affected == game.active_player.opponent().rook_start(castling_side) {
                    game.deny_castling(game.active_player.opponent(), castling_side);
                }
            }
        }

        // handle en passant target
        if let Move {
            kind: MoveKind::Pawn(PawnMove::DoubleStep),
            destination,
            ..
        } = mov
        {
            let en_passant_target = (destination + game.active_player.backwards_one_row())
                .expect("this cannot be outside the board");
            game.en_passant_target = Some(en_passant_target);
        } else {
            game.en_passant_target = None;
        }

        let future = game.clone().with_opponent_active();
        if future.legal_moves().count() == 0 {
            return if future.board.is_king_checked(future.active_player) {
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

            if game.fifty_move_rule_clock == FIFTY_MOVE_RULE_COUNT {
                return StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::FiftyMove),
                    final_game_state: game,
                });
            }
        }

        let piece_counts = game.board.piece_counts();

        if piece_counts == PieceCounts::KINGS_ONLY {
            return StepResult::Terminated(GameResult {
                kind: GameResultKind::Draw(DrawKind::InsufficientMaterial),
                final_game_state: game,
            });
        }

        if game.active_player == PlayerKind::Black {
            game.full_move_count.increase();
        }
        game.active_player = game.active_player.opponent();
        StepResult::Continued(game)
    }

    #[must_use]
    pub const fn apply_move_to_board(mut self, m: Move) -> Self {
        self.board.mov(m.origin, m.destination);
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
                self.board[affected] = None;
            }

            MoveKind::Pawn(PawnMove::Promotion { replacement, .. }) => {
                self.board[m.destination] = Some(Piece {
                    kind: replacement,
                    owner: self.active_player,
                });
            }

            MoveKind::King(KingMove::Castle {
                rook_start,
                rook_target,
                castling_side: _,
            }) => {
                self.board.mov(rook_start, rook_target);
            }
        }
        self
    }

    pub fn legal_moves(&self) -> impl Iterator<Item = Move> + Clone + use<'_> {
        self.threatening_move_candidates()
            .chain(self.pawn_step_candidates())
            .chain(self.castle_candidates())
            .filter(move |mov| {
                self.clone()
                    .apply_move_to_board(*mov)
                    .board
                    .is_king_checked(self.active_player)
                    .not()
            })
    }

    fn castle_candidates(&self) -> impl Iterator<Item = Move> + Clone {
        [CastlingSide::Kingside, CastlingSide::Queenside]
            .into_iter()
            .filter(|castling_side| self.is_castling_allowed(*castling_side))
            .filter(|castling_side| {
                let threatened_squares = self
                    .board
                    .threatened_squares_by(self.active_player.opponent())
                    .collect::<Vec<_>>();

                self.active_player
                    .castling_non_check_needed_squares(*castling_side)
                    .iter()
                    .all(|castle_square| {
                        threatened_squares
                            .iter()
                            .all(|threatened_square| threatened_square != castle_square)
                    })
                    && self
                        .active_player
                        .castling_free_needed_squares(*castling_side)
                        .iter()
                        .all(|square| self.board[*square].is_none())
            })
            .map(|castling_side| Move {
                kind: MoveKind::King(KingMove::Castle {
                    rook_start: self.active_player.rook_start(castling_side),
                    rook_target: self.active_player.rook_castling_target(castling_side),
                    castling_side,
                }),
                origin: self.active_player.king_start(),
                destination: self.active_player.king_castling_target(castling_side),
            })
    }

    fn threatening_move_candidates(&self) -> impl Iterator<Item = Move> + Clone + use<'_> {
        self.board
            .threatening_moves_by(self.active_player)
            .flat_map(|threat| self.threat_to_move_candidate(threat))
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
                                replacement: *promotion_option,
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

    #[must_use]
    fn pawn_step_candidates(&self) -> Vec<Move> {
        let mut candidates = vec![];
        for square in Square::all() {
            let player = self.active_player;
            if !(self.board[square]
                == Some(Piece {
                    kind: PieceKind::Pawn,
                    owner: player,
                }))
            {
                continue;
            }

            let one_in_front = (square + self.active_player.forwards_one_row())
                .expect("a pawn cannot exist on the last row");

            if self.board[one_in_front].is_some() {
                continue; // pawns cant capture moving forward!
            }

            if one_in_front.row == self.active_player.pawn_promotion_row() {
                PieceKind::PROMOTION_OPTIONS
                    .iter()
                    .map(|promotion_option| Move {
                        kind: MoveKind::Pawn(PawnMove::Promotion {
                            is_capture: false,
                            replacement: *promotion_option,
                        }),
                        origin: square,
                        destination: one_in_front,
                    })
                    .collect_into(&mut candidates);
            } else {
                candidates.push(Move {
                    kind: MoveKind::Pawn(PawnMove::SimpleStep),
                    origin: square,
                    destination: one_in_front,
                });
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

            candidates.push(Move {
                kind: MoveKind::Pawn(PawnMove::DoubleStep),
                origin: square,
                destination: two_in_front,
            });
        }
        candidates
    }
}

#[derive(Default)]
pub struct SearchStats {
    pub continued_games: usize,
    pub checkmated_games: usize,
    pub drawn_games: usize,
    pub en_passant: usize,
}

#[derive(Default)]
pub struct WalkStats {
    pub final_depth: u32,
    pub final_game_state: GameState,
    pub move_history: Vec<Vec<AsciiChar>>,
}

#[cfg(test)]
mod tests {
    use crate::notation::algebraic::long_algebraic_notation;
    use crate::notation::algebraic::standard_algebraic_notation;

    use super::*;
    use std::dbg;
    use std::io::Write;
    use std::print;
    use std::println;

    #[test]
    fn search() {
        crate::testing::skip_if_no_expensive_test_opt_in!();

        let depth = 3;
        let game = GameState::default();

        let before = std::time::Instant::now();

        let stats = game.search(depth, |_| ());

        println!("---------------------------");
        println!("depth: {depth}");
        println!("#checkmate: {}", stats.checkmated_games);
        println!("#drawn games: {}", stats.drawn_games);
        println!("#continued games: {}", stats.continued_games);
        println!("#en passant: {}", stats.en_passant);
        println!("elapsed: {:?}", before.elapsed());
        println!("---------------------------");
    }

    #[cfg(feature = "rand")]
    #[test]
    fn many_random_walks() {
        //crate::testing::skip_if_no_expensive_test_opt_in!();

        let max_depth = 1_000;
        let walk_count = 100;
        let game = GameState::default();

        for i in 0..walk_count {
            let stats = game.clone().random_walk(max_depth, owl_checker_depth_1);
            println!("{i}: {}", stats.final_depth);
            if stats.final_depth > max_depth - 300 {
                for (i, mov) in stats.move_history.chunks(2).enumerate() {
                    print!("{}. ", i + 1);
                    for m in mov {
                        print!("{} ", m.as_str());
                    }
                }
                std::io::stdout().flush().unwrap();
                panic!()
            }
        }
    }

    #[test]
    fn test_mass_fens() {
        crate::testing::skip_if_no_expensive_test_opt_in!();

        let fens = std::fs::read_to_string("./fens/fens.txt").unwrap();

        for fen in fens.lines() {
            let schach_game = GameState::try_from_fen(fen).unwrap();
            let schach_fen = schach_game.to_fen();
            assert_eq!(fen, schach_fen.as_str());
        }
    }

    #[test]
    fn test_mass_against_owl() {
        crate::testing::skip_if_no_expensive_test_opt_in!();

        let max_depth = 2;
        let max_fens = 10;
        let skip_fens = 100;
        let progress_thingy = core::cmp::max(max_fens / 1_000, 1);
        let fens = std::fs::read_to_string("./fens/fens.txt").unwrap();

        let mut progress = 0;

        for fen in fens.lines().skip(skip_fens).take(max_fens) {
            let mut game = GameState::try_from_fen(fen).unwrap();
            game.rule_set = RuleSet::Perft;
            let _ = game.search(max_depth, owl_checker_move_count);
            progress += 1;
            if progress % progress_thingy == 0 {
                println!("{progress}/{max_fens}");
            }
        }
    }

    fn owl_checker_move_count(game: &GameState) {
        let schach_move_count = game.legal_moves().count();
        let owl_move_count = owlchess::movegen::legal::gen_all(
            &owlchess::Board::from_fen(game.to_fen().as_str()).unwrap(),
        )
        .len();
        assert_eq!(schach_move_count, owl_move_count);
    }

    fn owl_checker_depth_1(game: &GameState) {
        let schach_all_legals = game.legal_moves().collect::<Vec<_>>();
        for mov in &schach_all_legals {
            let schach_move_san = standard_algebraic_notation(game.clone(), *mov);
            let owl_board = owlchess::Board::from_fen(game.to_fen().as_str()).unwrap();
            let owl_move =
                owlchess::Move::from_san(dbg!(schach_move_san.as_str()), &owl_board).unwrap();
            let new_owl_board = owl_board.make_move(owl_move).unwrap();
            let StepResult::Continued(new_schach_board) =
                game.clone().step(*mov, schach_all_legals.clone())
            else {
                continue;
            };

            let binding = new_schach_board.to_fen();
            let new_schach_fen = binding.as_str();
            let new_owl_fen = new_owl_board.as_fen();

            println!("schach: {new_schach_fen}");
            println!("owl___: {new_owl_fen}");

            let new_owl_move_count = owlchess::movegen::legal::gen_all(&new_owl_board)
                .iter()
                .count();

            let new_schach_legals = new_schach_board.legal_moves().collect::<Vec<_>>();

            for mov in &new_schach_legals {
                println!(
                    "lan: {}  san: {}",
                    long_algebraic_notation(new_schach_board.clone(), *mov).as_str(),
                    standard_algebraic_notation(new_schach_board.clone(), *mov).as_str()
                );
            }
            let new_schach_move_count = new_schach_legals.len();

            assert_eq!(new_schach_move_count, new_owl_move_count);
        }
    }
}
