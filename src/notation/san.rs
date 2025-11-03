use crate::game::CastlingSide;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::GameState;
use crate::game::StepResult;
use crate::mov::KingMove;
use crate::mov::Move;
use crate::mov::PawnMove;
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

    #[rustfmt::skip]
    let thingy = match mov {
        Move::Pawn(
            PawnMove::SimpleStep { start, target } | PawnMove::DoubleStep { start, target },
        ) => todo!(),
        Move::Pawn(PawnMove::EnPassant { start, target, affected }) => todo!(),
        Move::Pawn(PawnMove::Promotion { start, target, replacement, is_capture }) => todo!(),
        Move::Pawn(PawnMove::SimpleCapture { start, target }) => todo!(),
        | Move::Knight { start, target, is_capture, }
        | Move::Bishop { start, target, is_capture, }
        | Move::Rook { start, target, is_capture, }
        | Move::Queen { start, target, is_capture, }
        | Move::King( 
            KingMove::Normal { start, target, is_capture } 
          ) => todo!(),
        Move::King(KingMove::Castle(CastlingSide::Kingside)) => vec![
            AsciiChar::CapitalO,
            AsciiChar::HyphenMinus,
            AsciiChar::CapitalO,
        ],
        Move::King(KingMove::Castle(CastlingSide::Queenside)) => vec![
            AsciiChar::CapitalO,
            AsciiChar::HyphenMinus,
            AsciiChar::CapitalO,
            AsciiChar::HyphenMinus,
            AsciiChar::CapitalO,
        ], 
    };

    let outcome = game.step(*mov, legal_moves);

    let mut append = vec![];
    match outcome {
        | StepResult::Terminated(GameResult {
            kind: GameResultKind::Win,
            ..
        }) => {
            append.push(AsciiChar::NumberSign);
        }
        | StepResult::Terminated(GameResult {
            kind: GameResultKind::Draw(..),
            ..
        }) => {}
        | StepResult::Continued(future) => {
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
