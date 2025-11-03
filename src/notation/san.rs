use crate::board::Board;
use crate::coord::Square;
use crate::game::CastlingSide;
use crate::game::GameState;
use crate::game::{self};
use crate::mov::Move;
use crate::piece::Piece;
use crate::piece::PieceKind;
use alloc::vec;
use alloc::vec::Vec;
use core::ascii::Char as AsciiChar;

#[must_use]
pub fn thingy(game: GameState, mov: &Move) -> Vec<AsciiChar> {
    let legal_moves: Vec<Move> = game.legal_moves().collect();
    assert!(legal_moves.iter().any(|m| m == mov));

    let owner = game.active_player;

    let thingy = match mov {
        Move::Normal {
            piece_kind,
            start,
            target,
            is_capture,
        } => {
            let piece = match piece_kind {
                PieceKind::Pawn => vec![],
                _ => vec![
                    Piece::to_ascii_char(Piece {
                        kind: *piece_kind,
                        owner,
                    })
                    .to_uppercase(),
                ],
            };
            let start = start.to_fen();
            let target = target.to_fen();
            let capture = if *is_capture {
                vec![AsciiChar::SmallX]
            } else {
                vec![]
            };
            [piece, start, capture, target].concat()
        }
        Move::DoubleStep { start, target } => [start.to_fen(), target.to_fen()].concat(),
        Move::Promotion {
            start,
            target,
            is_capture,
            replacement,
        } => {
            let start = start.to_fen();
            let target = target.to_fen();
            let capture = if *is_capture {
                vec![AsciiChar::SmallX]
            } else {
                vec![]
            };
            let replacement = vec![
                Piece::to_ascii_char(Piece {
                    kind: *replacement,
                    owner,
                })
                .to_uppercase(),
            ];
            let equals = vec![AsciiChar::EqualsSign];
            [start, capture, target, equals, replacement].concat()
        }
        Move::EnPassant {
            start,
            target,
            affected_square: _,
        } => {
            let start = start.to_fen();
            let target = target.to_fen();
            let capture = vec![AsciiChar::SmallX];
            [start, capture, target].concat()
        }
        Move::Castling(CastlingSide::Kingside) => {
            vec![
                AsciiChar::CapitalO,
                AsciiChar::HyphenMinus,
                AsciiChar::CapitalO,
            ]
        }
        Move::Castling(CastlingSide::Queenside) => {
            vec![
                AsciiChar::CapitalO,
                AsciiChar::HyphenMinus,
                AsciiChar::CapitalO,
                AsciiChar::HyphenMinus,
                AsciiChar::CapitalO,
            ]
        }
    };

    let outcome = game.step(*mov, legal_moves);

    let appendage = match outcome {
        game::StepResult::Terminated(game_result) => todo!(),
        game::StepResult::Continued(game_state) => todo!(),
    };

    todo!()
}
