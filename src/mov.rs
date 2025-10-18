use crate::{
    coord::Square,
    game::CastlingSide,
    piece::{Piece, PieceKind},
};

#[derive(Copy, Clone, PartialEq, Eq, strum::Display)]
pub enum Move {
    Normal {
        piece_kind: PieceKind,
        start: Square,
        target: Square,
        is_capture: bool,
    },
    DoubleStep {
        start: Square,
        target: Square,
    },
    Promotion {
        start: Square,
        target: Square,
        is_capture: bool,
        replacement: PieceKind,
    },
    EnPassant {
        start: Square,
        target: Square,
        affected_square: Square,
    },
    Castling(CastlingSide),
}
impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal {
                piece_kind,
                start,
                target,
                is_capture,
            } => {
                write!(f, "{piece_kind:?}: {start:?} -> {target:?} {is_capture}",)
            }
            Self::DoubleStep { start, target } => write!(f, "Double Step: {start:?} -> {target:?}"),
            Self::Promotion {
                start,
                target,
                is_capture,
                replacement,
            } => write!(
                f,
                "Promotion: {start:?} -> {target:?} to: {replacement} {is_capture}",
            ),
            Self::EnPassant {
                start,
                target,
                affected_square,
            } => write!(
                f,
                "En Passant: {start:?} -> {target:?}, affected: {affected_square:?}",
            ),
            Self::Castling(castling_side) => write!(f, "{castling_side}",),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Threat {
    pub piece: Piece,
    pub start: Square,
    pub target: Square,
}

impl std::fmt::Debug for Threat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?} => {:?}", self.piece, self.start, self.target)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnPassantTargetSquare {
    pub inner: Square,
    pub half_turn_round: u64,
}
