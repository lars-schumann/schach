use super::*;

#[test]
fn test() {
    let starting_square = Square {
        col: Col::C3,
        row: Row::R4,
    };
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
    board.place(starting_square, piece);
    let mut attacking_moves: Vec<Threat> = vec![];

    for col in Col::COLS {
        for row in Row::ROWS {
            let square = Square { col, row };
            if (board.lookup(square)).is_some_and(|piece| piece.owner == active_player) {
                attacking_moves.append(&mut attacked_squares(&board, square, active_player));
            }
        }
    }

    let dbg_board = DebugBoard {
        inner: board,
        attacked_squares: attacking_moves
            .iter()
            .map(|threat| threat.target_square)
            .collect(),
    };
    dbg!(attacking_moves);
    dbg!(dbg_board);
}
