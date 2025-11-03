use crate::game::CastlingSide;
use crate::game::GameResult;
use crate::game::GameResultKind;
use crate::game::GameState;
use crate::game::StepResult;
use crate::mov::NewMove;
use crate::piece::Piece;
use crate::piece::PieceKind;
use alloc::vec;
use alloc::vec::Vec;
use core::ascii::Char as AsciiChar;
use std::println;

#[must_use]
pub fn thingy(game: GameState, mov: &NewMove) {
    let legal_moves: Vec<NewMove> = game.legal_moves().collect();
    assert!(legal_moves.iter().any(|m| m == mov));

    let owner = game.active_player;

    let thingy = match mov {
        | _ => todo!(),
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
