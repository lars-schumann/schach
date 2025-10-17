use crate::coord::{Offset, Row, Square};
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
            (Self::White, CastlingSide::Kingside) => vec![Square::E1, Square::F1, Square::G1],
            (Self::White, CastlingSide::Queenside) => {
                vec![Square::E1, Square::D1, Square::C1, Square::B1]
            }
            (Self::Black, CastlingSide::Kingside) => vec![Square::E8, Square::F8, Square::G8],
            (Self::Black, CastlingSide::Queenside) => {
                vec![Square::E8, Square::D8, Square::C8, Square::B8]
            }
        }
    }

    #[must_use]
    pub const fn king_start(&self) -> Square {
        match self {
            Self::White => Square::E1,
            Self::Black => Square::E8,
        }
    }

    #[must_use]
    pub const fn king_castling_target(&self, castling_side: CastlingSide) -> Square {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => Square::G1,
            (Self::White, CastlingSide::Queenside) => Square::C1,
            (Self::Black, CastlingSide::Kingside) => Square::G8,
            (Self::Black, CastlingSide::Queenside) => Square::C8,
        }
    }

    #[must_use]
    pub const fn rook_start(&self, castling_side: CastlingSide) -> Square {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => Square::H1,
            (Self::White, CastlingSide::Queenside) => Square::A1,
            (Self::Black, CastlingSide::Kingside) => Square::H8,
            (Self::Black, CastlingSide::Queenside) => Square::A8,
        }
    }

    #[must_use]
    pub const fn rook_castling_target(&self, castling_side: CastlingSide) -> Square {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => Square::F1,
            (Self::White, CastlingSide::Queenside) => Square::D1,
            (Self::Black, CastlingSide::Kingside) => Square::F8,
            (Self::Black, CastlingSide::Queenside) => Square::D8,
        }
    }

    #[must_use]
    pub const fn forwards_one_row(&self) -> Offset {
        match self {
            Self::White => Offset::U,
            Self::Black => Offset::D,
        }
    }

    #[must_use]
    pub const fn backwards_one_row(&self) -> Offset {
        match self {
            Self::White => Offset::D,
            Self::Black => Offset::U,
        }
    }
}
