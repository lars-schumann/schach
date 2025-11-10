use crate::coord::Col;
use crate::coord::Row;
use crate::coord::Square;
use crate::coord::Square as S;
use crate::game::PieceCounts;
use crate::game::attacked_squares;
use crate::mov::Threat;
use crate::piece::Piece;
use crate::piece::PieceKind;
use crate::player::PlayerKind;
use alloc::vec::Vec;

pub static COL_COUNT: usize = 8;
pub static ROW_COUNT: usize = 8;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board(pub [[Option<Piece>; ROW_COUNT]; COL_COUNT]);
impl Board {
    #[must_use]
    pub const fn empty() -> Self {
        Self([[None; ROW_COUNT]; COL_COUNT])
    }

    pub(crate) fn threatening_moves_by(
        &self,
        threatened_by: PlayerKind,
    ) -> impl Iterator<Item = Threat> {
        S::ALL
            .iter()
            .flat_map(move |square| crate::game::attacked_squares(self, *square, threatened_by))
    }

    pub fn threatened_squares_by(&self, threatened_by: PlayerKind) -> impl Iterator<Item = S> {
        S::ALL.iter().flat_map(move |square| {
            attacked_squares(self, *square, threatened_by).map(|threat| threat.destination)
        })
    }

    #[must_use]
    pub fn king_position(&self, king_owner: PlayerKind) -> S {
        *S::ALL
            .iter()
            .find(|square| {
                self[**square]
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

    pub const fn mov(&mut self, start: S, target: S) {
        self[target] = self[start];
        self[start] = None;
    }

    #[must_use]
    pub fn new() -> Self {
        let mut board = Self::empty();

        board[S::A1] = Some(Piece::WHITE_ROOK);
        board[S::B1] = Some(Piece::WHITE_KNIGHT);
        board[S::C1] = Some(Piece::WHITE_BISHOP);
        board[S::D1] = Some(Piece::WHITE_QUEEN);
        board[S::E1] = Some(Piece::WHITE_KING);
        board[S::F1] = Some(Piece::WHITE_BISHOP);
        board[S::G1] = Some(Piece::WHITE_KNIGHT);
        board[S::H1] = Some(Piece::WHITE_ROOK);

        board[S::A8] = Some(Piece::BLACK_ROOK);
        board[S::B8] = Some(Piece::BLACK_KNIGHT);
        board[S::C8] = Some(Piece::BLACK_BISHOP);
        board[S::D8] = Some(Piece::BLACK_QUEEN);
        board[S::E8] = Some(Piece::BLACK_KING);
        board[S::F8] = Some(Piece::BLACK_BISHOP);
        board[S::G8] = Some(Piece::BLACK_KNIGHT);
        board[S::H8] = Some(Piece::BLACK_ROOK);

        for col in Col::ALL {
            board[S { col, row: Row::R2 }] = Some(Piece::WHITE_PAWN);
            board[S { col, row: Row::R7 }] = Some(Piece::BLACK_PAWN);
        }

        board
    }

    #[must_use]
    pub fn piece_counts(&self) -> PieceCounts {
        let mut piece_counts = PieceCounts::default();
        for square in Square::ALL {
            if let Some(piece) = self[square] {
                piece_counts[piece] += 1;
            }
        }
        piece_counts
    }
}
impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
impl const core::ops::Index<S> for Board {
    type Output = Option<Piece>;
    fn index(&self, index: S) -> &Self::Output {
        let col = usize::from(u8::from(index.col) - 1);
        let row = usize::from(u8::from(index.row) - 1);
        &self.0[col][row]
    }
}
impl const core::ops::IndexMut<S> for Board {
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        let col = usize::from(u8::from(index.col) - 1);
        let row = usize::from(u8::from(index.row) - 1);
        &mut self.0[col][row]
    }
}
impl core::fmt::Debug for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f)?;
        for square in Square::ALL {
            if square.col == Col::C1 {
                write!(f, "{}", i8::from(square.row))?;
            }
            match self[square] {
                Some(piece) => write!(f, "{piece} ")?,
                None => {
                    if square.is_black() {
                        write!(f, "□ ",)?;
                    } else {
                        write!(f, "■ ",)?;
                    }
                }
            }
            if square.col == Col::C8 {
                writeln!(f)?;
            }
        }
        write!(f, "  ")?;
        for col in Col::ALL {
            write!(f, "{} ", i8::from(col))?;
        }
        Ok(())
    }
}

#[allow(unused)]
pub(crate) struct DebugBoard {
    pub inner: Board,
    pub highlighted_squares: Vec<S>,
}
impl core::fmt::Debug for DebugBoard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f)?;
        for square in Square::ALL {
            if square.col == Col::C1 {
                write!(f, "{}", i8::from(square.row))?;
            }
            if self.highlighted_squares.contains(&square) {
                write!(f, "\x1B[31m",)?;
            }
            match self.inner[square] {
                Some(piece) => write!(f, "{piece} ")?,
                None => {
                    if square.is_black() {
                        write!(f, "□ ",)?;
                    } else {
                        write!(f, "■ ",)?;
                    }
                }
            }
            if self.highlighted_squares.contains(&square) {
                write!(f, "\x1B[0m",)?;
            }
            if square.col == Col::C8 {
                writeln!(f)?;
            }
        }
        write!(f, "  ")?;
        for col in Col::ALL {
            write!(f, "{} ", i8::from(col))?;
        }
        Ok(())
    }
}
