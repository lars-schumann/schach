use crate::coord::Square;
use crate::game::CastlingSide;
use crate::game::DrawKind;
use crate::game::FIFTY_MOVE_RULE_COUNT;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::GameState;
use crate::game::Position;
use crate::game::REPETITIONS_TO_FORCED_DRAW_COUNT;
use crate::game::StepResult;
use crate::mov::KingMove;
use crate::mov::Move;
use crate::mov::PawnMove;
use crate::mov::Threat;
use crate::piece::Piece;
use crate::piece::PieceKind;
use crate::player::PlayerKind;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Not;

impl GameState {
    #[must_use]
    pub fn search(self, max_depth: u32) -> SearchStats {
        let mut terminated_games_checkmate: Vec<Self> = vec![];
        let mut terminated_games_draw: Vec<Self> = vec![];
        let mut continued_games: Vec<Self> = vec![self];
        let mut new_continued_games: Vec<Self> = vec![];

        let mut stats: SearchStats = SearchStats::default();

        for _ in 0..=max_depth {
            continued_games.clone().into_iter().for_each(|game| {
                let legal_moves: Vec<Move> = game.legal_moves().collect();

                for mov in legal_moves.clone() {
                    if matches!(mov, Move::Pawn(PawnMove::EnPassant { .. })) {
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

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn step(self, mov: Move, all_legal_moves: Vec<Move>) -> StepResult {
        let mut game = self.apply_move_to_board(mov);

        let current_position = Position {
            board: game.board,
            possible_moves: all_legal_moves,
        };

        // handle fifty move rule
        match (mov.piece_kind(), mov.is_capture()) {
            | (PieceKind::Pawn, _) | (_, true) => game.fifty_move_rule_clock.reset(),
            | _ => game.fifty_move_rule_clock.increase(),
        }

        // handle our own castling rights
        match mov {
            Move::King(_) => {
                game.deny_castling(game.active_player, CastlingSide::Kingside);
                game.deny_castling(game.active_player, CastlingSide::Queenside);
            }
            Move::Rook { start, .. } => {
                for castling_side in [CastlingSide::Kingside, CastlingSide::Queenside] {
                    if start == game.active_player.rook_start(castling_side) {
                        game.deny_castling(game.active_player, castling_side);
                    }
                }
            }
            | _ => { /*nothing */ }
        }

        // handle opponents castling rights
        match mov.capture_affected_square() {
            | Some(affected) => {
                for castling_side in [CastlingSide::Kingside, CastlingSide::Queenside] {
                    if affected == game.active_player.opponent().rook_start(castling_side) {
                        game.deny_castling(game.active_player.opponent(), castling_side);
                    }
                }
            }
            | None => { /*nothing */ }
        }

        // handle en passant target
        if let Move::Pawn(PawnMove::DoubleStep { target, .. }) = mov {
            let en_passant_target = (target + game.active_player.backwards_one_row())
                .expect("this cannot be outside the board");
            game.en_passant_target = Some(en_passant_target);
        } else {
            game.en_passant_target = None;
        }

        if game.is_perft.not() {
            game.position_history.push(current_position.clone());

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

        if game.active_player == PlayerKind::Black {
            game.full_move_count.increase();
        }
        game.active_player = game.active_player.opponent();
        StepResult::Continued(game)
    }

    #[must_use]
    pub const fn apply_move_to_board(mut self, m: Move) -> Self {
        match m {
            | Move::Pawn(
                PawnMove::SimpleStep { start, target }
                | PawnMove::DoubleStep { start, target }
                | PawnMove::SimpleCapture { start, target },
            )
            | Move::Knight { start, target, .. }
            | Move::Bishop { start, target, .. }
            | Move::Rook { start, target, .. }
            | Move::Queen { start, target, .. }
            | Move::King(KingMove::Normal { start, target, .. }) => {
                self.board.mov(start, target);
            }

            Move::Pawn(PawnMove::EnPassant {
                start,
                target,
                affected,
            }) => {
                self.board.mov(start, target);
                self.board[affected] = None;
            }

            Move::Pawn(PawnMove::Promotion {
                start,
                target,
                replacement,
                ..
            }) => {
                self.board.mov(start, target);
                self.board[target] = Some(Piece {
                    kind: replacement,
                    owner: self.active_player,
                });
            }

            Move::King(KingMove::Castle(castling_side)) => {
                self.board.mov(
                    self.active_player.king_start(),
                    self.active_player.king_castling_target(castling_side),
                );
                self.board.mov(
                    self.active_player.rook_start(castling_side),
                    self.active_player.rook_castling_target(castling_side),
                );
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
            .map(|castling_side| Move::King(KingMove::Castle(castling_side)))
    }

    fn threatening_move_candidates(&self) -> impl Iterator<Item = Move> + Clone + use<'_> {
        self.board
            .threatening_moves_by(self.active_player)
            .flat_map(|threat| self.threat_to_move_candidate(threat))
    }

    #[must_use]
    fn threat_to_move_candidate(&self, threat: Threat) -> Vec<Move> {
        match (threat.piece.kind, self.board[threat.target]) {
            | (PieceKind::Knight, target_square) => vec![Move::Knight {
                start: threat.start,
                target: threat.target,
                is_capture: target_square.is_some(),
            }],
            | (PieceKind::Bishop, target_square) => vec![Move::Bishop {
                start: threat.start,
                target: threat.target,
                is_capture: target_square.is_some(),
            }],
            | (PieceKind::Rook, target_square) => vec![Move::Rook {
                start: threat.start,
                target: threat.target,
                is_capture: target_square.is_some(),
            }],
            | (PieceKind::Queen, target_square) => vec![Move::Queen {
                start: threat.start,
                target: threat.target,
                is_capture: target_square.is_some(),
            }],
            | (PieceKind::King, target_square) => vec![Move::King(KingMove::Normal {
                start: threat.start,
                target: threat.target,
                is_capture: target_square.is_some(),
            })],
            | (PieceKind::Pawn, Some(_)) => {
                if threat.target.row == self.active_player.pawn_promotion_row() {
                    PieceKind::PROMOTION_OPTIONS
                        .iter()
                        .map(|promotion_option| {
                            Move::Pawn(PawnMove::Promotion {
                                start: threat.start,
                                target: threat.target,
                                is_capture: true,
                                replacement: *promotion_option,
                            })
                        })
                        .collect()
                } else {
                    vec![Move::Pawn(PawnMove::SimpleCapture {
                        start: threat.start,
                        target: threat.target,
                    })]
                }
            }
            | (PieceKind::Pawn, None) => {
                //en passant case, this is never gonna lead to promotion
                if let Some(en_passant_target) = self.en_passant_target
                    && threat.target == en_passant_target
                {
                    vec![Move::Pawn(PawnMove::EnPassant {
                        start: threat.start,
                        target: threat.target,
                        affected: (threat.target + self.active_player.backwards_one_row())
                            .expect("how is this not on the board?"),
                    })]
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
                    .map(|promotion_option| {
                        Move::Pawn(PawnMove::Promotion {
                            start: square,
                            target: one_in_front,
                            is_capture: false,
                            replacement: *promotion_option,
                        })
                    })
                    .collect_into(&mut candidates);
            } else {
                candidates.push(Move::Pawn(PawnMove::SimpleStep {
                    start: square,
                    target: one_in_front,
                }));
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

            candidates.push(Move::Pawn(PawnMove::DoubleStep {
                start: square,
                target: two_in_front,
            }));
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn search() {
        crate::testing::bail_if_no_expensive_test_opt_in!();

        let depth = 3;
        let game = GameState::try_from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();

        let before = std::time::Instant::now();

        let stats = game.search(depth);

        println!("---------------------------");
        println!("depth: {depth}");
        println!("#checkmate: {}", stats.checkmated_games);
        println!("#drawn games: {}", stats.drawn_games);
        println!("#continued games: {}", stats.continued_games);
        println!("#en passant: {}", stats.en_passant);
        println!("elapsed: {:?}", before.elapsed());
        println!("---------------------------");
    }

    #[test]
    fn test_mass_fens() {
        crate::testing::bail_if_no_expensive_test_opt_in!();

        let fens = std::fs::read_to_string("./fens/fens.txt").unwrap();

        for fen in fens.lines() {
            let schach_game = GameState::try_from_fen(fen).unwrap();
            let schach_fen = schach_game.to_fen();
            assert_eq!(fen, schach_fen.as_str());
        }
    }

    #[test]
    fn test_mass_against_owl() {
        crate::testing::bail_if_no_expensive_test_opt_in!();

        let max_depth = 2;
        let max_fens = 10;
        let skip_fens = 100;
        let progress_thingy = core::cmp::max(max_fens / 1_000, 1);
        let fens = std::fs::read_to_string("./fens/fens.txt").unwrap();

        let mut progress = 0;

        for fen in fens.lines().skip(skip_fens).take(max_fens) {
            let mut game = GameState::try_from_fen(fen).unwrap();
            game.is_perft = true;
            let _ = game.search(max_depth);
            progress += 1;
            if progress % progress_thingy == 0 {
                println!("{progress}/{max_fens}");
            }
        }
    }
}
