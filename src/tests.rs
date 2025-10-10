use crate::game::*;

#[test]
fn test() {
    let starting_square = Square(Col::C3, Row::R4);
    let white = PlayerKind::White;
    let black = PlayerKind::Black;
    test_piece(
        Piece {
            kind: PieceKind::Pawn,
            owner: white,
        },
        starting_square,
        white,
    );
    test_piece(
        Piece {
            kind: PieceKind::Pawn,
            owner: black,
        },
        starting_square,
        black,
    );
    test_piece(
        Piece {
            kind: PieceKind::Knight,
            owner: white,
        },
        starting_square,
        white,
    );
    test_piece(
        Piece {
            kind: PieceKind::Bishop,
            owner: white,
        },
        starting_square,
        white,
    );
    test_piece(
        Piece {
            kind: PieceKind::Rook,
            owner: white,
        },
        starting_square,
        white,
    );
    test_piece(
        Piece {
            kind: PieceKind::Queen,
            owner: white,
        },
        starting_square,
        white,
    );
    test_piece(
        Piece {
            kind: PieceKind::King,
            owner: white,
        },
        starting_square,
        white,
    );
}
fn test_piece(piece: Piece, starting_square: Square, active_player: PlayerKind) {
    let mut board = Board::new();
    board[starting_square] = Some(piece);
    for row in Row::ROWS {
        board[Square(Col::C6, row)] = Some(Piece {
            kind: PieceKind::Pawn,
            owner: PlayerKind::Black,
        });
    }

    let mut attacking_moves: Vec<Threat> = vec![];

    for col in Col::COLS {
        for row in Row::ROWS {
            let square = Square(col, row);
            if (board[square]).is_some_and(|piece| piece.owner == active_player) {
                attacking_moves.append(&mut attacked_squares(&board, square, active_player));
            }
        }
    }

    let dbg_board = DebugBoard {
        inner: board,
        attacked_squares: attacking_moves.iter().map(|threat| threat.target).collect(),
    };
    dbg!(attacking_moves);
    dbg!(dbg_board);
}

fn attacked_squares(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> Vec<Threat> {
    let Some(piece) = board[starting_square] else {
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
            match board[target_square] {
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
            start: starting_square,
            target: target_square,
        })
        .collect()
}
