use crate::game::CastlingSide;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::GameState;
use crate::game::StepResult;
use crate::mov::KingMove;
use crate::mov::Move;
use crate::mov::PawnMove;
use alloc::vec;
use alloc::vec::Vec;
use core::ascii::Char as AsciiChar;

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

#[must_use]
pub fn long_algebraic(game: GameState, mov: &Move) -> Vec<AsciiChar> {
    let legal_moves: Vec<Move> = game.legal_moves().collect();
    assert!(legal_moves.iter().any(|m| m == mov));

    let active_player = game.active_player;

    let core_move_notation = match mov {
        Move::Pawn(
            PawnMove::SimpleStep { start, target }
            | PawnMove::DoubleStep { start, target }
            | PawnMove::SimpleCapture { start, target }
            | PawnMove::EnPassant { start, target, .. }
            | PawnMove::Promotion { start, target, .. },
        )
        | Move::Knight { start, target, .. }
        | Move::Bishop { start, target, .. }
        | Move::Rook { start, target, .. }
        | Move::Queen { start, target, .. }
        | Move::King(KingMove::Normal { start, target, .. }) => {
            let piece_representation = matches!(mov, Move::Pawn(_))
                .then_some([mov.piece_kind().to_white_piece().to_fen_repr()]);

            let start = start.to_fen_repr();

            let capture_symbol = mov.is_capture().then_some([AsciiChar::SmallX]);

            let target = target.to_fen_repr();

            let promotion_replacement = match mov {
                Move::Pawn(PawnMove::Promotion { replacement, .. }) => {
                    Some([replacement.to_white_piece().to_fen_repr()])
                }
                _ => None,
            };

            #[rustfmt::skip]
            [
                piece_representation.as_ref().map_or_default(<[_; 1]>::as_slice),
                start.as_slice(),
                capture_symbol.as_ref().map_or_default(<[_; 1]>::as_slice),
                target.as_slice(),
                promotion_replacement.as_ref().map_or_default(<[_; 1]>::as_slice),
            ]
            .concat()
        }
        Move::King(KingMove::Castle(CastlingSide::Kingside)) => O_O.to_vec(),
        Move::King(KingMove::Castle(CastlingSide::Queenside)) => O_O_O.to_vec(),
    };

    let outcome = game.step(*mov, legal_moves);

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn test_thingy() {
        let game = GameState::new();
        let legal_moves = game.legal_moves();
        for mov in legal_moves {
            println!("{:?}", long_algebraic(game.clone(), &mov));
        }
    }
}
