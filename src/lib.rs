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
    directions: &[Offset],
    range_upper_bound: Option<i32>,
) -> impl Iterator<Item = Square> + use<> {
    let sqr = board.lookup(starting_square);
    assert!(sqr.is_some_and(|piece| piece.owner == active_player));

    let rays = directions.iter().map(move |direction| {
        (0..range_upper_bound.unwrap_or(i32::MAX))
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
}
fn find_attacking_moves(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    let Some(piece) = board.lookup(starting_square) else {
        return vec![].into_iter();
    };
    if piece.owner != active_player {
        return vec![].into_iter();
    }

    match piece.kind {
        PieceKind::Pawn => pawn_attacking_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
        PieceKind::Knight => knight_attacking_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
        PieceKind::Bishop => bishop_attacking_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
        PieceKind::Rook => rook_attacking_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
        PieceKind::Queen => queen_attacking_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
        PieceKind::King => king_attacking_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
    }
}

fn knight_attacking_moves(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::Knight,
            owner: active_player
        })
    );
    attacked_squares(
        board,
        starting_square,
        active_player,
        &Offset::KNIGHT,
        Some(1),
    )
    .map(move |target_square| match board.lookup(target_square) {
        None => Move {
            kind: MoveKind::Knight(KnightMove::Move),
            start: starting_square,
            end: target_square,
        },
        Some(piece) if piece.owner != active_player => Move {
            kind: MoveKind::Knight(KnightMove::Capture),
            start: starting_square,
            end: target_square,
        },
        _ => unreachable!(),
    })
}

fn bishop_attacking_moves(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::Bishop,
            owner: active_player
        })
    );
    attacked_squares(board, starting_square, active_player, &Offset::BISHOP, None).map(
        move |target_square| match board.lookup(target_square) {
            None => Move {
                kind: MoveKind::Bishop(BishopMove::Move),
                start: starting_square,
                end: target_square,
            },
            Some(piece) if piece.owner != active_player => Move {
                kind: MoveKind::Bishop(BishopMove::Capture),
                start: starting_square,
                end: target_square,
            },
            _ => unreachable!(),
        },
    )
}

fn rook_attacking_moves(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::Rook,
            owner: active_player
        })
    );
    attacked_squares(board, starting_square, active_player, &Offset::ROOK, None).map(
        move |target_square| match board.lookup(target_square) {
            None => Move {
                kind: MoveKind::Rook(RookMove::Move),
                start: starting_square,
                end: target_square,
            },
            Some(piece) if piece.owner != active_player => Move {
                kind: MoveKind::Rook(RookMove::Capture),
                start: starting_square,
                end: target_square,
            },
            _ => unreachable!(),
        },
    )
}

fn queen_attacking_moves(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::Queen,
            owner: active_player
        })
    );
    attacked_squares(board, starting_square, active_player, &Offset::QUEEN, None).map(
        move |target_square| match board.lookup(target_square) {
            None => Move {
                kind: MoveKind::Queen(QueenMove::Move),
                start: starting_square,
                end: target_square,
            },
            Some(piece) if piece.owner != active_player => Move {
                kind: MoveKind::Queen(QueenMove::Capture),
                start: starting_square,
                end: target_square,
            },
            _ => unreachable!(),
        },
    )
}

fn king_attacking_moves(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::King,
            owner: active_player
        })
    );
    attacked_squares(
        board,
        starting_square,
        active_player,
        &Offset::KING_DIRECT,
        Some(1),
    )
    .map(move |target_square| match board.lookup(target_square) {
        None => Move {
            kind: MoveKind::King(KingMove::Move),
            start: starting_square,
            end: target_square,
        },
        Some(piece) if piece.owner != active_player => Move {
            kind: MoveKind::King(KingMove::Capture),
            start: starting_square,
            end: target_square,
        },
        _ => unreachable!(),
    })
}

fn pawn_attacking_moves(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::Pawn,
            owner: active_player
        })
    );
    let (forward_diagonal, promotion_row) = if active_player == PlayerKind::White {
        (Offset::PAWN_UP_DIAGONAL, Row::R8)
    } else {
        (Offset::PAWN_DOWN_DIAGONAL, Row::R1)
    };

    attacked_squares(
        board,
        starting_square,
        active_player,
        &forward_diagonal,
        Some(1),
    )
    .filter_map(move |target_square| match board.lookup(target_square) {
        None => None,
        Some(piece) if piece.owner != active_player => {
            if target_square.row == promotion_row {
                Some(Move {
                    kind: MoveKind::Pawn(PawnMove::CapturePromote),
                    start: starting_square,
                    end: target_square,
                })
            } else {
                Some(Move {
                    kind: MoveKind::Pawn(PawnMove::Capture),
                    start: starting_square,
                    end: target_square,
                })
            }
        }
        _ => unreachable!(),
    })
}

pub fn game() {
    let mut board = Board::new();
    board.place(
        Square {
            col: Col::C4,
            row: Row::R1,
        },
        Piece {
            kind: PieceKind::Rook,
            owner: PlayerKind::White,
        },
    );
    let mut attacking_moves: Vec<Move> = vec![];

    for col in Col::COLS {
        for row in Row::ROWS {
            let square = Square { col, row };
            if (board.lookup(square)).is_some_and(|piece| piece.owner == PlayerKind::White) {
                attacking_moves
                    .append(&mut find_attacking_moves(&board, square, PlayerKind::White).collect());
            }
        }
    }

    dbg!(attacking_moves);
}
