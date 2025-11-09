use crate::game::CastlingSide;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::GameState;
use crate::game::StepResult;
use crate::mov::KingMove;
use crate::mov::Move;
use crate::mov::MoveKind;
use crate::mov::PawnMove;
use crate::piece::PieceKind;
use alloc::vec;
use alloc::vec::Vec;
use core::ascii::Char as AsciiChar;
use core::ops::Not;

const O_O: [AsciiChar; 3] = [
    AsciiChar::CapitalO,
    AsciiChar::HyphenMinus,
    AsciiChar::CapitalO,
];

const O_O_O: [AsciiChar; 5] = [
    AsciiChar::CapitalO,
    AsciiChar::HyphenMinus,
    AsciiChar::CapitalO,
    AsciiChar::HyphenMinus,
    AsciiChar::CapitalO,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbiguationLevel {
    OriginEmpty,
    OriginFileOnly,
    OriginRankOnly,
    OriginFull,
}

#[derive(Debug, Clone, Copy)]
pub struct CaptureRepresentation {
    capture: Option<AsciiChar>,
    no_capture: Option<AsciiChar>,
}

#[must_use]
fn notation_creator(
    game: GameState,
    mov: Move,
    ambiguation_level: AmbiguationLevel,
    capture_representation: CaptureRepresentation,
) -> Vec<AsciiChar> {
    let legal_moves: Vec<Move> = game.legal_moves().collect();
    assert!(legal_moves.iter().any(|m| m == &mov));

    let active_player = game.active_player;

    let core_move_notation = match mov.kind {
        MoveKind::King(KingMove::Castle {
            castling_side: CastlingSide::Kingside,
            ..
        }) => O_O.to_vec(),
        MoveKind::King(KingMove::Castle {
            castling_side: CastlingSide::Queenside,
            ..
        }) => O_O_O.to_vec(),
        MoveKind::Pawn(_)
        | MoveKind::Knight { .. }
        | MoveKind::Bishop { .. }
        | MoveKind::Rook { .. }
        | MoveKind::Queen { .. }
        | MoveKind::King(KingMove::Normal { .. }) => {
            let piece_repr = (matches!(mov.kind, MoveKind::Pawn(_))).not().then_some([mov
                .kind
                .piece_kind()
                .to_white_piece()
                .to_fen_repr()]);

            let start_square_repr = match ambiguation_level {
                AmbiguationLevel::OriginEmpty => [].to_vec(),
                AmbiguationLevel::OriginFileOnly => [mov.origin.col.to_fen_repr()].to_vec(),
                AmbiguationLevel::OriginRankOnly => [mov.origin.row.to_fen_repr()].to_vec(),
                AmbiguationLevel::OriginFull => mov.origin.to_fen_repr().to_vec(),
            };

            let capture_symbol = if mov.is_capture() {
                capture_representation.capture.map(|c| [c])
            } else {
                capture_representation.no_capture.map(|c| [c])
            };

            let target = mov.destination.to_fen_repr();

            let promotion_replacement = match mov.kind {
                MoveKind::Pawn(PawnMove::Promotion { replacement, .. }) => {
                    Some([replacement.to_white_piece().to_fen_repr()])
                }
                _ => None,
            };

            #[rustfmt::skip]
            [
                piece_repr.as_ref().map_or_default(<[_; 1]>::as_slice),
                start_square_repr.as_slice(),
                capture_symbol.as_ref().map_or_default(<[_; 1]>::as_slice),
                target.as_slice(),
                promotion_replacement.as_ref().map_or_default(<[_; 1]>::as_slice),
            ]
            .concat()
        }
    };

    let outcome = game.step(mov, legal_moves);

    let mut append = vec![];
    match outcome {
        | StepResult::Continued(future) => {
            if future.board.is_king_checked(active_player.opponent()) {
                append.push(AsciiChar::PlusSign);
            }
        }
        | StepResult::Terminated(GameResult {
            kind: GameResultKind::Win,
            ..
        }) => {
            append.push(AsciiChar::NumberSign);
        }
        | StepResult::Terminated(GameResult {
            kind: GameResultKind::Draw(_),
            ..
        }) => { /* TODO: nothing yet, this isn't Ascii :[ 1/2 / 1/2 or smt */ }
    }

    [core_move_notation, append].concat()
}

#[must_use]
pub fn long_algebraic_notation(game: GameState, mov: Move) -> Vec<AsciiChar> {
    notation_creator(
        game,
        mov,
        AmbiguationLevel::OriginFull,
        CaptureRepresentation {
            capture: Some(AsciiChar::SmallX),
            no_capture: Some(AsciiChar::HyphenMinus),
        },
    )
}

#[must_use]
pub fn standard_algebraic_notation(game: GameState, mov: Move) -> Vec<AsciiChar> {
    let capture_repr = CaptureRepresentation {
        capture: Some(AsciiChar::SmallX),
        no_capture: None,
    };
    let mut legal_moves = game.legal_moves().collect::<Vec<_>>();
    let mov_index = legal_moves
        .iter()
        .enumerate()
        .find(|m| *m.1 == mov)
        .expect("passed illegal move")
        .0;

    legal_moves.swap_remove(mov_index);

    let interfering_moves = legal_moves
        .iter()
        .filter(|legal| legal.kind.piece_kind() == mov.kind.piece_kind())
        .filter(|legal| legal.destination == mov.destination)
        .collect::<Vec<_>>();

    if interfering_moves.is_empty() {
        if mov.kind.piece_kind() == PieceKind::Pawn && mov.is_capture() {
            //Pawns always have the File when capturing!
            return notation_creator(game, mov, AmbiguationLevel::OriginFileOnly, capture_repr);
        }
        return notation_creator(game, mov, AmbiguationLevel::OriginEmpty, capture_repr);
    }

    if interfering_moves
        .iter()
        .any(|inter| inter.origin.row == mov.origin.row)
        .not()
    {
        return notation_creator(game, mov, AmbiguationLevel::OriginRankOnly, capture_repr);
    }

    if interfering_moves
        .iter()
        .any(|inter| inter.origin.col == mov.origin.col)
        .not()
    {
        return notation_creator(game, mov, AmbiguationLevel::OriginFileOnly, capture_repr);
    }

    notation_creator(game, mov, AmbiguationLevel::OriginFull, capture_repr)
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
            println!("{:?}", standard_algebraic_notation(game.clone(), mov));
        }
    }
}
