use alloc::vec;
use alloc::vec::Vec;
use core::ops::Not;

use crate::board::Board;
use crate::coord::Square;
use crate::game::CastlingSide;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::GameState;
use crate::game::GameStateCore;
use crate::game::StepResult;
use crate::mv::KingMove;
use crate::mv::Move;
use crate::mv::MoveKind;
use crate::mv::PawnMove;
use crate::mv::Threat;
use crate::piece::PieceKind;

impl GameState {
    #[must_use]
    pub fn search(self, max_depth: u32, checker: impl Fn(&Self)) -> SearchStats {
        let mut terminated_games_checkmate: Vec<Self> = vec![];
        let mut terminated_games_draw: Vec<Self> = vec![];
        let mut continued_games: Vec<Self> = vec![self];
        let mut new_continued_games: Vec<Self> = vec![];

        for _ in 0..=max_depth {
            continued_games.clone().into_iter().for_each(|game| {
                checker(&game);
                let legal_moves: Vec<Move> = game.core.legal_moves().collect();

                for mv in legal_moves.clone() {
                    match game.clone().step(mv, legal_moves.clone()) {
                        StepResult::Terminated(GameResult {
                            kind: GameResultKind::Win,
                            final_game_state,
                        }) => terminated_games_checkmate.push(final_game_state),
                        StepResult::Terminated(GameResult {
                            kind: GameResultKind::Draw(_),
                            final_game_state,
                        }) => terminated_games_draw.push(final_game_state),
                        StepResult::Continued(game_state) => {
                            new_continued_games.push(game_state);
                        }
                    }
                }
            });

            core::mem::swap(&mut continued_games, &mut new_continued_games);
            new_continued_games.clear();
        }

        SearchStats {
            continued_games: continued_games.len(),
            checkmated_games: terminated_games_checkmate.len(),
            drawn_games: terminated_games_draw.len(),
        }
    }

    #[cfg(feature = "rand")]
    pub fn random_walk(self, max_depth: u32, checker: impl Fn(&Self)) -> StepResult {
        use crate::alloc::borrow::ToOwned as _;

        let mut rng = rand::rng();
        let mut game = self;

        for _ in 0..max_depth {
            use rand::seq::IndexedRandom;

            checker(&game);
            let legal_moves: Vec<Move> = game.core.legal_moves().collect();

            let random_move = legal_moves
                .choose(&mut rng)
                .expect("If the game has no legal moves, it should've ended last turn")
                .to_owned();

            match game.clone().step(random_move, legal_moves.clone()) {
                StepResult::Continued(game_state) => {
                    game = game_state;
                }
                terminated @ StepResult::Terminated(_) => {
                    return terminated;
                }
            }
        }
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
            .flat_map(|threat| self.threat_to_move_candidates(threat))
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
    fn threat_to_move_candidates(&self, threat: Threat) -> Vec<Move> {
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
}

#[cfg(test)]
mod tests {
    use std::println;

    use super::*;
    use crate::coord::square::*;
    use crate::game::GameStateCore;
    use crate::notation::san::standard_algebraic_notation;
    use crate::piece::Piece;
    use crate::player::PlayerKind;
    use crate::testing::skip_if_no_expensive_test_opt_in;

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
        let game = GameState::new();

        for i in 0..walk_count {
            let final_step = game.clone().random_walk(max_depth, owl_checker_depth_1);
            println!("{i}: {}", final_step.game_state().core.full_move_count.0);
        }
    }

    #[allow(dead_code)]
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
        for mv in &schach_all_legals {
            let schach_move_san = standard_algebraic_notation(game.clone(), *mv);
            let owl_board = owlchess::Board::from_fen(game.core.to_fen().as_str()).unwrap();
            let owl_move = owlchess::Move::from_san(schach_move_san.as_str(), &owl_board).unwrap();

            let new_owl_board = owl_board.make_move(owl_move).unwrap();
            let StepResult::Continued(new_schach_board) =
                game.clone().step(*mv, schach_all_legals.clone())
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

    #[test]
    fn test_pawn_attacked_squares() {
        let mut board = Board::empty();

        board[E2] = Some(Piece::WHITE_PAWN);

        #[rustfmt::skip]
        let expected_attacked = [
        //  A8,  B8,  C8,  D8,  E8,  F8,  G8,  H8,
        //  A7,  B7,  C7,  D7,  E7,  F7,  G7,  H7,
        //  A6,  B6,  C6,  D6,  E6,  F6,  G6,  H6,
        //  A5,  B5,  C5,  D5,  E5,  F5,  G5,  H5,
        //  A4,  B4,  C4,  D4,  E4,  F4,  G4,  H4,
        /*  A3,  B3,  C3,*/D3,/*E3,*/F3,//G3,  H3,
        //  A2,  B2,  C2,  D2,  E2,  F2,  G2,  H2,
        //  A1,  B1,  C1,  D1,  E1,  F1,  G1,  H1,
        ];

        compare_expected_to_actual_attacked(&expected_attacked, board, PlayerKind::White);
    }

    #[test]
    fn test_multiple_pawn_attacked_squares() {
        let mut board = Board::empty();

        board[E2] = Some(Piece::WHITE_PAWN);
        board[E3] = Some(Piece::WHITE_PAWN);
        board[B7] = Some(Piece::WHITE_PAWN);

        #[rustfmt::skip]
        let expected_attacked = [
            A8,/*B8,*/C8,//D8,  E8,  F8,  G8,  H8,
        //  A7,  B7,  C7,  D7,  E7,  F7,  G7,  H7,
        //  A6,  B6,  C6,  D6,  E6,  F6,  G6,  H6,
        //  A5,  B5,  C5,  D5,  E5,  F5,  G5,  H5,
        /*  A4,  B4,  C4,*/D4,/*E4,*/F4,//G4,  H4,
        /*  A3,  B3,  C3,*/D3,/*E3,*/F3,//G3,  H3,
        //  A2,  B2,  C2,  D2,  E2,  F2,  G2,  H2,
        //  A1,  B1,  C1,  D1,  E1,  F1,  G1,  H1,
        ];

        compare_expected_to_actual_attacked(&expected_attacked, board, PlayerKind::White);
    }

    #[test]
    fn test_multiple_knight_attacked_squares() {
        let mut board = Board::empty();

        board[B1] = Some(Piece::WHITE_KNIGHT);
        board[H1] = Some(Piece::WHITE_KNIGHT);
        board[D6] = Some(Piece::WHITE_KNIGHT);

        #[rustfmt::skip]
        let expected_attacked = [
        /*  A8,  B8,*/C8,/*D8,*/E8,//F8,  G8,  H8,
        /*  A7,*/B7,/*C7,  D7,  E7,*/F7,//G7,  H7,
        //  A6,  B6,  C6,  D6,  E6,  F6,  G6,  H6,
        /*  A5,*/B5,/*C5,  D5,  E5,*/F5,//G5,  H5,
        /*  A4,  B4,*/C4,/*D4,*/E4,//F4,  G4,  H4,
            A3,/*B3,*/C3,/*D3,  E3,  F3,*/G3,//H3,
        /*  A2,  B2,  C2,*/D2,/*E2,*/F2,//G2,  H2,
        //  A1,  B1,  C1,  D1,  E1,  F1,  G1,  H1,
        ];

        compare_expected_to_actual_attacked(&expected_attacked, board, PlayerKind::White);
    }

    fn compare_expected_to_actual_attacked(
        expected_attacked: &[Square],
        board: Board,
        player: PlayerKind,
    ) {
        let core_game = GameStateCore {
            board,
            ..Default::default()
        };

        let attacked_squares = core_game
            .board
            .threatened_squares_by(player)
            .collect::<Vec<_>>();

        assert_eq!(expected_attacked, attacked_squares);
    }
}

// #[rustfmt::skip]
//         let expected_attacked = [
//         //  A8,  B8,  C8,  D8,  E8,  F8,  G8,  H8,
//         //  A7,  B7,  C7,  D7,  E7,  F7,  G7,  H7,
//         //  A6,  B6,  C6,  D6,  E6,  F6,  G6,  H6,
//         //  A5,  B5,  C5,  D5,  E5,  F5,  G5,  H5,
//         //  A4,  B4,  C4,  D4,  E4,  F4,  G4,  H4,
//         //  A3,  B3,  C3,  D3,  E3,  F3,  G3,  H3,
//         //  A2,  B2,  C2,  D2,  E2,  F2,  G2,  H2,
//         //  A1,  B1,  C1,  D1,  E1,  F1,  G1,  H1,
//         ];
