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

    pub const fn mov(&mut self, start: Square, target: Square) {
        self[target] = self[start];
        self[start] = None;
    }

    #[must_use]
    pub fn filled(with_pawns: bool) -> Self {
        let mut board = Self::new();

        board[Square::A1] = Some(Piece::ROOK_WHITE);
        board[Square::B1] = Some(Piece::KNIGHT_WHITE);
        board[Square::C1] = Some(Piece::BISHOP_WHITE);
        board[Square::D1] = Some(Piece::QUEEN_WHITE);
        board[Square::E1] = Some(Piece::KING_WHITE);
        board[Square::F1] = Some(Piece::BISHOP_WHITE);
        board[Square::G1] = Some(Piece::KNIGHT_WHITE);
        board[Square::H1] = Some(Piece::ROOK_WHITE);

        board[Square::A8] = Some(Piece::ROOK_BLACK);
        board[Square::B8] = Some(Piece::KNIGHT_BLACK);
        board[Square::C8] = Some(Piece::BISHOP_BLACK);
        board[Square::D8] = Some(Piece::QUEEN_BLACK);
        board[Square::E8] = Some(Piece::KING_BLACK);
        board[Square::F8] = Some(Piece::BISHOP_BLACK);
        board[Square::G8] = Some(Piece::KNIGHT_BLACK);
        board[Square::H8] = Some(Piece::ROOK_BLACK);

        if with_pawns {
            for col in Col::COLS {
                board[Square { col, row: Row::R2 }] = Some(Piece::PAWN_WHITE);
                board[Square { col, row: Row::R7 }] = Some(Piece::PAWN_BLACK);
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
        let col = usize::from(index.col) - 1;
        let row = usize::from(index.row) - 1;
        &self.0[col][row]
    }
}
impl const std::ops::IndexMut<Square> for Board {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        let col = usize::from(index.col) - 1;
        let row = usize::from(index.row) - 1;
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
