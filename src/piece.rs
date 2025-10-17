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
    pub const fn new(kind: PieceKind, owner: PlayerKind) -> Self {
        Self { kind, owner }
    }
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

    pub const PAWN_WHITE: Self = Self::new(PieceKind::Pawn, PlayerKind::White);
    pub const KNIGHT_WHITE: Self = Self::new(PieceKind::Knight, PlayerKind::White);
    pub const BISHOP_WHITE: Self = Self::new(PieceKind::Bishop, PlayerKind::White);
    pub const ROOK_WHITE: Self = Self::new(PieceKind::Rook, PlayerKind::White);
    pub const QUEEN_WHITE: Self = Self::new(PieceKind::Queen, PlayerKind::White);
    pub const KING_WHITE: Self = Self::new(PieceKind::King, PlayerKind::White);

    pub const PAWN_BLACK: Self = Self::new(PieceKind::Pawn, PlayerKind::Black);
    pub const KNIGHT_BLACK: Self = Self::new(PieceKind::Knight, PlayerKind::Black);
    pub const BISHOP_BLACK: Self = Self::new(PieceKind::Bishop, PlayerKind::Black);
    pub const ROOK_BLACK: Self = Self::new(PieceKind::Rook, PlayerKind::Black);
    pub const QUEEN_BLACK: Self = Self::new(PieceKind::Queen, PlayerKind::Black);
    pub const KING_BLACK: Self = Self::new(PieceKind::King, PlayerKind::Black);
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match *self {
            Self::PAWN_WHITE => "♙",
            Self::KNIGHT_WHITE => "♘",
            Self::BISHOP_WHITE => "♗",
            Self::ROOK_WHITE => "♖",
            Self::QUEEN_WHITE => "♕",
            Self::KING_WHITE => "♔",

            Self::PAWN_BLACK => "♟",
            Self::KNIGHT_BLACK => "♞",
            Self::BISHOP_BLACK => "♝",
            Self::ROOK_BLACK => "♜",
            Self::QUEEN_BLACK => "♛",
            Self::KING_BLACK => "♚",
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
