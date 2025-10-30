use crate::coord::{Offset, Row, Square as S};
use crate::game::CastlingSide as CS;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum PlayerKind {
    #[default]
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
    pub const fn castling_non_check_needed_squares(self, castling_side: CS) -> [S; 3] {
        match (self, castling_side) {
            (Self::White, CS::Kingside) => [S::E1, S::F1, S::G1],
            (Self::White, CS::Queenside) => [S::E1, S::D1, S::C1],
            (Self::Black, CS::Kingside) => [S::E8, S::F8, S::G8],
            (Self::Black, CS::Queenside) => [S::E8, S::D8, S::C8],
        }
    }

    #[must_use]
    pub fn castling_free_needed_squares(self, castling_side: CS) -> Vec<S> {
        match (self, castling_side) {
            (Self::White, CS::Kingside) => vec![S::F1, S::G1],
            (Self::White, CS::Queenside) => {
                vec![S::D1, S::C1, S::B1]
            }
            (Self::Black, CS::Kingside) => vec![S::F8, S::G8],
            (Self::Black, CS::Queenside) => {
                vec![S::D8, S::C8, S::B8]
            }
        }
    }

    #[must_use]
    pub const fn king_start(&self) -> S {
        match self {
            Self::White => S::E1,
            Self::Black => S::E8,
        }
    }

    #[must_use]
    pub const fn king_castling_target(&self, castling_side: CS) -> S {
        match (self, castling_side) {
            (Self::White, CS::Kingside) => S::G1,
            (Self::White, CS::Queenside) => S::C1,
            (Self::Black, CS::Kingside) => S::G8,
            (Self::Black, CS::Queenside) => S::C8,
        }
    }

    #[must_use]
    pub const fn rook_start(&self, castling_side: CS) -> S {
        match (self, castling_side) {
            (Self::White, CS::Kingside) => S::H1,
            (Self::White, CS::Queenside) => S::A1,
            (Self::Black, CS::Kingside) => S::H8,
            (Self::Black, CS::Queenside) => S::A8,
        }
    }

    #[must_use]
    pub const fn rook_castling_target(&self, castling_side: CS) -> S {
        match (self, castling_side) {
            (Self::White, CS::Kingside) => S::F1,
            (Self::White, CS::Queenside) => S::D1,
            (Self::Black, CS::Kingside) => S::F8,
            (Self::Black, CS::Queenside) => S::D8,
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
        self.forwards_one_row() * -1
    }
}
