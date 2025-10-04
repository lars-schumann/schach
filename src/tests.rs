use super::*;

#[test]
fn test_game() {
    game();
}

#[test]
fn test_rook() {
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
