use crate::coord::Square;
use crate::game::CastlingSide;
use crate::piece::Piece;
use crate::piece::PieceKind;

#[derive(Clone, Copy)]
pub (crate) struct Threat {
    pub (crate) piece: Piece,
    pub (crate) start: Square,
    pub (crate) target: Square,
}

impl core::fmt::Debug for Threat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {:?} => {:?}", self.piece, self.start, self.target)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnPassantTargetSquare {
    pub inner: Square,
    pub half_turn_round: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    Pawn(PawnMove),
    Knight {
        start: Square,
        target: Square,
        is_capture: bool,
    },
    Bishop {
        start: Square,
        target: Square,
        is_capture: bool,
    },
    Rook {
        start: Square,
        target: Square,
        is_capture: bool,
    },
    Queen {
        start: Square,
        target: Square,
        is_capture: bool,
    },
    King(KingMove),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PawnMove {
    SimpleStep {
        start: Square,
        target: Square,
    },
    DoubleStep {
        start: Square,
        target: Square,
    },
    SimpleCapture {
        start: Square,
        target: Square,
    },
    EnPassant {
        start: Square,
        target: Square,
        affected: Square,
    },
    Promotion {
        start: Square,
        target: Square,
        replacement: PieceKind,
        is_capture: bool,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KingMove {
    Normal {
        start: Square,
        target: Square,
        is_capture: bool,
    },
    Castle(CastlingSide),
}

impl Move {
    #[must_use]
    pub const fn is_capture(&self) -> bool {
        match self {
            | Self::Knight { is_capture, .. }
            | Self::Bishop { is_capture, .. }
            | Self::Rook { is_capture, .. }
            | Self::Queen { is_capture, .. }
            | Self::King(KingMove::Normal { is_capture, .. })
            | Self::Pawn(PawnMove::Promotion { is_capture, .. }) => *is_capture,
            | Self::Pawn(PawnMove::SimpleCapture { .. } | PawnMove::EnPassant { .. }) => true,
            | Self::Pawn(PawnMove::SimpleStep { .. } | PawnMove::DoubleStep { .. })
            | Self::King(KingMove::Castle(_)) => false,
        }
    }

    #[must_use]
    #[rustfmt::skip]
    pub const fn capture_affected_square(&self) -> Option<Square> {
        match self {
            | Self::Pawn(
                | PawnMove::SimpleCapture { target, .. }
                | PawnMove::Promotion { is_capture: true, target, .. }
                | PawnMove::EnPassant { affected: target, .. }
            )
            | Self::Knight { is_capture: true, target, .. }
            | Self::Bishop { is_capture: true, target, .. }
            | Self::Rook { is_capture: true, target, .. }
            | Self::Queen { is_capture: true, target, .. }
            | Self::King(
                KingMove::Normal { is_capture: true, target, .. }
            ) => Some(*target),
            | Self::Pawn(
                | PawnMove::SimpleStep { .. }
                | PawnMove::DoubleStep { .. }
                | PawnMove::Promotion { is_capture: false, .. },
            )
            | Self::Knight { is_capture: false, .. }
            | Self::Bishop { is_capture: false, .. }
            | Self::Rook { is_capture: false, .. }
            | Self::Queen { is_capture: false, .. }
            | Self::King(
                | KingMove::Normal { is_capture: false, .. } 
                | KingMove::Castle(_)
            ) => None,
        }
    }

    #[must_use]
    pub const fn piece_kind(&self) -> PieceKind {
        match self {
            | Self::Pawn(_) => PieceKind::Pawn,
            | Self::Knight { .. } => PieceKind::Knight,
            | Self::Bishop { .. } => PieceKind::Bishop,
            | Self::Rook { .. } => PieceKind::Rook,
            | Self::Queen { .. } => PieceKind::Queen,
            | Self::King(_) => PieceKind::King,
        }
    }
}
