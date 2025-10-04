#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![feature(const_trait_impl, const_ops, const_convert, const_try)]
#[allow(clippy::wildcard_imports)]
use crate::game::*;

pub mod game;

#[cfg(test)]
mod tests;

fn attacked_squares(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> Vec<Threat> {
    let Some(piece) = board.lookup(starting_square) else {
        return vec![];
    };
    if piece.owner != active_player {
        return vec![];
    }
    let (directions, range_upper_bound) = piece.threat_directions();
    let range_upper_bound = i32::from(range_upper_bound);

    let rays = directions.iter().map(move |direction| {
        (0..range_upper_bound)
            .map(move |i| starting_square + *direction * (i + 1))
            .take_while(Result::is_ok) //}
            .map(Result::unwrap) //} FIXME: uh this cant be right
    });

    let mut out = vec![];
    for ray in rays {
        for target_square in ray {
            match board.lookup(target_square) {
                None => out.push(target_square),
                Some(piece) if piece.owner == active_player => {
                    break;
                }
                Some(piece) if piece.owner != active_player => {
                    out.push(target_square);
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
    out.into_iter()
        .map(|target_square| Threat {
            piece,
            starting_square,
            target_square,
        })
        .collect()
}
