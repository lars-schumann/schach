use crate::coord::Square;
use crate::game::CastlingSide;
use crate::piece::Piece;
use crate::piece::PieceKind;

#[derive(Clone, Copy)]
pub (crate) struct Threat {
    pub (crate) piece: Piece,
    pub (crate) origin: Square,
    pub (crate) destination: Square,
}

impl core::fmt::Debug for Threat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {:?} => {:?}", self.piece, self.origin, self.destination)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveKind {
    Pawn(PawnMove),
    Knight {is_capture: bool,},
    Bishop {
        is_capture: bool,
    },
    Rook {
        is_capture: bool,
    },
    Queen {
        is_capture: bool,
    },
    King(KingMove),
}

impl MoveKind{
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


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move{
    pub kind: MoveKind,
    pub origin: Square,
    pub destination: Square,

}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PawnMove {
    SimpleStep,
    DoubleStep,
    SimpleCapture,
    EnPassant {affected: Square,},
    Promotion {
        replacement: PieceKind,
        is_capture: bool,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KingMove {
    Normal {
        
        is_capture: bool,
    },
    Castle{rook_start: Square, rook_target: Square, castling_side: CastlingSide},
}

impl Move {
    #[must_use]
    pub const fn is_capture(&self) -> bool {
        match self.kind {
            | MoveKind::Knight { is_capture, .. }
            | MoveKind::Bishop { is_capture, .. }
            | MoveKind::Rook { is_capture, .. }
            | MoveKind::Queen { is_capture, .. }
            | MoveKind::King(KingMove::Normal { is_capture, .. })
            | MoveKind::Pawn(PawnMove::Promotion { is_capture, .. }) => is_capture,
            | MoveKind::Pawn(PawnMove::SimpleCapture { .. } | PawnMove::EnPassant { .. }) => true,
            | MoveKind::Pawn(PawnMove::SimpleStep { .. } | PawnMove::DoubleStep { .. })
            | MoveKind::King(KingMove::Castle{..}) => false,
        }
    }

    #[must_use]
    #[rustfmt::skip]
    pub const fn capture_affected_square(&self) -> Option<Square> {
        
        if let MoveKind::Pawn(PawnMove::EnPassant { affected }) = self.kind{
            return Some(affected);
        }
        if self.is_capture(){
            Some(self.destination)
        } else{
            None
        }
    }
}
