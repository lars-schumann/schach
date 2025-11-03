use crate::game::CastlingSide;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::GameState;
use crate::game::StepResult;
use crate::mov::Move;
use crate::piece::Piece;
use crate::piece::PieceKind;
use alloc::vec;
use alloc::vec::Vec;
use core::ascii::Char as AsciiChar;
use std::println;

#[must_use]
pub fn thingy(game: GameState, mov: &Move) {
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

    let mut append = vec![];
    match outcome {
        StepResult::Terminated(GameResult {
            kind: GameResultKind::Win,
            ..
        }) => append.push(AsciiChar::NumberSign),
        StepResult::Terminated(GameResult {
            kind: GameResultKind::Draw(..),
            ..
        }) => {}
        StepResult::Continued(future) => {
            if future.board.is_king_checked(owner.opponent()) {
                append.push(AsciiChar::PlusSign);
            }
        }
    }

    println!("{:?}", [thingy, append].concat());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn test_thingy() {
        let game = GameState::new();
        let legal_moves = game.legal_moves();
        for mov in legal_moves {
            thingy(game.clone(), &mov);
        }
    }
}
