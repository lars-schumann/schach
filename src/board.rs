pub static COL_COUNT: usize = 8;
pub static ROW_COUNT: usize = 8;

use crate::coord::{Col, Row, Square};
use crate::game::attacked_squares;
use crate::mov::Threat;
use crate::piece::{Piece, PieceKind};
use crate::player::PlayerKind;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board(pub [[Option<Piece>; ROW_COUNT]; COL_COUNT]);
impl Board {
    #[must_use]
    pub const fn new() -> Self {
        Self([[None; ROW_COUNT]; COL_COUNT])
    }

    pub fn threatening_moves_by(
        &self,
        threatened_by: PlayerKind,
    ) -> impl Iterator<Item = Threat> + Clone + use<'_> {
        Square::all()
            .flat_map(move |square| crate::game::attacked_squares(self, square, threatened_by))
    }

    pub fn threatened_squares_by(&self, threatened_by: PlayerKind) -> impl Iterator<Item = Square> {
        Square::all().flat_map(move |square| {
            attacked_squares(self, square, threatened_by)
                .into_iter()
                .map(|threat| threat.target)
        })
    }

    #[must_use]
    pub fn king_position(&self, king_owner: PlayerKind) -> Square {
        Square::all()
            .find(|square| {
                self[*square]
                    == Some(Piece {
                        kind: PieceKind::King,
                        owner: king_owner,
                    })
            })
            .expect("where did the king go?")
    }

    #[must_use]
    pub fn is_king_checked(&self, king_owner: PlayerKind) -> bool {
        self.threatened_squares_by(king_owner.opponent())
            .any(|square| square == self.king_position(king_owner))
    }

    #[allow(clippy::cast_sign_loss)]
    pub const fn movee(&mut self, start: Square, target: Square) {
        let start_col = usize::from(start.col);
        let start_row = usize::from(start.row);
        let target_col = usize::from(target.col);
        let target_row = usize::from(target.row);
        self.0[target_col][target_row] = self.0[start_col][start_row];
        self.0[start_col][start_row] = None;
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn filled(with_pawns: bool) -> Self {
        use PieceKind::{Pawn, Knight, Bishop, Rook, King, Queen};
        use PlayerKind::{White, Black};
        use Row::{R2, R7, };
      
        let mut board = Self::new();

        board[Square::A1] = Some(Piece{kind: Rook,   owner: White});
        board[Square::B1] = Some(Piece{kind: Knight, owner: White});
        board[Square::C1] = Some(Piece{kind: Bishop, owner: White});
        board[Square::D1] = Some(Piece{kind: Queen,  owner: White});
        board[Square::E1] = Some(Piece{kind: King,   owner: White});
        board[Square::F1] = Some(Piece{kind: Bishop, owner: White});
        board[Square::G1] = Some(Piece{kind: Knight, owner: White});
        board[Square::H1] = Some(Piece{kind: Rook,   owner: White});

        board[Square::A8] = Some(Piece{kind: Rook,   owner: Black});
        board[Square::B8] = Some(Piece{kind: Knight, owner: Black});
        board[Square::C8] = Some(Piece{kind: Bishop, owner: Black});
        board[Square::D8] = Some(Piece{kind: Queen,  owner: Black});
        board[Square::E8] = Some(Piece{kind: King,   owner: Black});
        board[Square::F8] = Some(Piece{kind: Bishop, owner: Black});
        board[Square::G8] = Some(Piece{kind: Knight, owner: Black});
        board[Square::H8] = Some(Piece{kind: Rook,   owner: Black});

        if with_pawns{
            for col in Col::COLS {
                board[Square{ col,  row: R2 }] = Some( Piece { kind: Pawn, owner: White });
                board[Square{ col,  row: R7 }] = Some( Piece { kind: Pawn, owner: Black });
            }
        }

        board
    }
}
impl Default for Board {
    fn default() -> Self {
        Self::filled(true)
    }
}
impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in Row::ROWS.into_iter().rev() {
            write!(f, "{}", i32::from(row) + 1)?;
            for col in Col::COLS {
                match self[Square { col, row }] {
                    None => {
                        if (i32::from(row) + i32::from(col)) % 2 == 0 {
                            write!(f, "□ ",)?;
                        } else {
                            write!(f, "■ ",)?;
                        }
                    }
                    Some(piece) => write!(f, "{piece} ")?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "  ")?;
        for col in Col::COLS {
            write!(f, "{} ", i32::from(col) + 1)?;
        }
        Ok(())
    }
}
impl const std::ops::Index<Square> for Board {
    type Output = Option<Piece>;
    fn index(&self, index: Square) -> &Self::Output {
        let col = usize::from(index.col);
        let row = usize::from(index.row);
        &self.0[col][row]
    }
}
impl const std::ops::IndexMut<Square> for Board {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        let col = usize::from(index.col);
        let row = usize::from(index.row);
        &mut self.0[col][row]
    }
}

pub struct DebugBoard {
    pub inner: Board,
    pub attacked_squares: Vec<Square>,
}
impl std::fmt::Debug for DebugBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in Row::ROWS.into_iter().rev() {
            write!(f, "{}", i32::from(row) + 1)?;
            for col in Col::COLS {
                if self.attacked_squares.contains(&Square { col, row }) {
                    write!(f, "\x1B[31m",)?;
                }
                match self.inner[Square { col, row }] {
                    None => {
                        if (i32::from(row) + i32::from(col)) % 2 == 0 {
                            write!(f, "□ ",)?;
                        } else {
                            write!(f, "■ ",)?;
                        }
                    }

                    Some(piece) => write!(f, "{piece} ")?,
                }
                if self.attacked_squares.contains(&Square { col, row }) {
                    write!(f, "\x1B[0m",)?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "  ")?;
        for col in Col::COLS {
            write!(f, "{} ", i32::from(col) + 1)?;
        }
        Ok(())
    }
}
