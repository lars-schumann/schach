use crate::coord::Offset;
use crate::player::PlayerKind;

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
impl PieceKind {
    pub const PROMOTION_OPTIONS: [Self; 4] = [Self::Knight, Self::Bishop, Self::Rook, Self::Queen];
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceKind,
    pub owner: PlayerKind,
}
impl Piece {
    #[must_use]
    pub const fn threat_directions(&self) -> (&[Offset], Range) {
        match (self.kind, self.owner) {
            (PieceKind::Pawn, PlayerKind::White) => (&Offset::PAWN_UP_DIAGONAL, Range::One),
            (PieceKind::Pawn, PlayerKind::Black) => (&Offset::PAWN_DOWN_DIAGONAL, Range::One),
            (PieceKind::Knight, _) => (&Offset::KNIGHT, Range::One),
            (PieceKind::Bishop, _) => (&Offset::BISHOP, Range::Unlimited),
            (PieceKind::Rook, _) => (&Offset::ROOK, Range::Unlimited),
            (PieceKind::Queen, _) => (&Offset::QUEEN, Range::Unlimited),
            (PieceKind::King, _) => (&Offset::KING_DIRECT, Range::One),
        }
    }

    pub const PAWN_WHITE: Self = Self {
        kind: PieceKind::Pawn,
        owner: PlayerKind::White,
    };

    
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self.owner {
            PlayerKind::White => match self.kind {
                PieceKind::Pawn => "♙",
                PieceKind::Knight => "♘",
                PieceKind::Bishop => "♗",
                PieceKind::Rook => "♖",
                PieceKind::Queen => "♕",
                PieceKind::King => "♔",
            },
            PlayerKind::Black => match self.kind {
                PieceKind::Pawn => "♟",
                PieceKind::Knight => "♞",
                PieceKind::Bishop => "♝",
                PieceKind::Rook => "♜",
                PieceKind::Queen => "♛",
                PieceKind::King => "♚",
            },
        };
        write!(f, "{symbol}")
    }
}

pub enum Range {
    One,
    Unlimited,
}
impl From<Range> for i32 {
    fn from(value: Range) -> Self {
        match value {
            Range::One => 1,
            Range::Unlimited => Self::MAX,
        }
    }
}
