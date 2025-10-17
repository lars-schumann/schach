use crate::coord::{Col, Offset, Row, Square};
use crate::game::CastlingSide;

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display)]
pub enum PlayerKind {
    White,
    Black,
}
impl PlayerKind {
    #[must_use]
    pub const fn opponent(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    #[must_use]
    pub const fn pawn_starting_row(self) -> Row {
        match self {
            Self::White => Row::R2,
            Self::Black => Row::R7,
        }
    }

    #[must_use]
    pub const fn pawn_promotion_row(self) -> Row {
        match self {
            Self::White => Row::R8,
            Self::Black => Row::R1,
        }
    }

    #[must_use]
    pub fn castling_non_check_needed_squares(self, castling_side: CastlingSide) -> Vec<Square> {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => vec![
                Square(Col::C5, Row::R1),
                Square(Col::C6, Row::R1),
                Square(Col::C7, Row::R1),
            ],
            (Self::White, CastlingSide::Queenside) => vec![
                Square(Col::C5, Row::R1),
                Square(Col::C4, Row::R1),
                Square(Col::C3, Row::R1),
                Square(Col::C2, Row::R1),
            ],
            (Self::Black, CastlingSide::Kingside) => vec![
                Square(Col::C5, Row::R8),
                Square(Col::C6, Row::R8),
                Square(Col::C7, Row::R8),
            ],
            (Self::Black, CastlingSide::Queenside) => vec![
                Square(Col::C5, Row::R8),
                Square(Col::C4, Row::R8),
                Square(Col::C3, Row::R8),
                Square(Col::C2, Row::R8),
            ],
        }
    }

    #[must_use]
    pub const fn king_start(&self) -> Square {
        match self {
            Self::White => Square(Col::C5, Row::R1),
            Self::Black => Square(Col::C5, Row::R8),
        }
    }

    #[must_use]
    pub const fn king_castling_target(&self, castling_side: CastlingSide) -> Square {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => Square(Col::C7, Row::R1),
            (Self::White, CastlingSide::Queenside) => Square(Col::C3, Row::R1),
            (Self::Black, CastlingSide::Kingside) => Square(Col::C7, Row::R8),
            (Self::Black, CastlingSide::Queenside) => Square(Col::C3, Row::R8),
        }
    }

    #[must_use]
    pub const fn rook_start(&self, castling_side: CastlingSide) -> Square {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => Square(Col::C8, Row::R1),
            (Self::White, CastlingSide::Queenside) => Square(Col::C1, Row::R1),
            (Self::Black, CastlingSide::Kingside) => Square(Col::C1, Row::R8),
            (Self::Black, CastlingSide::Queenside) => Square(Col::C8, Row::R8),
        }
    }

    #[must_use]
    pub const fn rook_castling_target(&self, castling_side: CastlingSide) -> Square {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => Square(Col::C6, Row::R1),
            (Self::White, CastlingSide::Queenside) => Square(Col::C4, Row::R1),
            (Self::Black, CastlingSide::Kingside) => Square(Col::C6, Row::R8),
            (Self::Black, CastlingSide::Queenside) => Square(Col::C4, Row::R8),
        }
    }

    #[must_use]
    pub const fn forwards_one_row(&self) -> Offset {
        match self {
            Self::White => Offset { col: 0, row: 1 },
            Self::Black => Offset { col: 0, row: -1 },
        }
    }

    #[must_use]
    pub const fn backwards_one_row(&self) -> Offset {
        match self {
            Self::White => Offset { col: 0, row: -1 },
            Self::Black => Offset { col: 0, row: 1 },
        }
    }
}
