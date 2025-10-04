use std::fmt::Debug;

pub static COL_COUNT: usize = 8;
pub static ROW_COUNT: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum Col {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
}
impl Col {
    pub const COLS: [Self; COL_COUNT] = [
        Self::C1,
        Self::C2,
        Self::C3,
        Self::C4,
        Self::C5,
        Self::C6,
        Self::C7,
        Self::C8,
    ];
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum Row {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}
impl Row {
    pub const ROWS: [Self; ROW_COUNT] = [
        Self::R1,
        Self::R2,
        Self::R3,
        Self::R4,
        Self::R5,
        Self::R6,
        Self::R7,
        Self::R8,
    ];
}
impl const std::ops::Add<i32> for Row {
    type Output = Result<Self, ()>;
    fn add(self, rhs: i32) -> Self::Output {
        let row_number: i32 = self.into();
        (row_number + rhs).try_into()
    }
}
impl const std::ops::Add<i32> for Col {
    type Output = Result<Self, ()>;
    fn add(self, rhs: i32) -> Self::Output {
        let column_number: i32 = self.into();
        (column_number + rhs).try_into()
    }
}
impl const TryFrom<i32> for Row {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::R1),
            1 => Ok(Self::R2),
            2 => Ok(Self::R3),
            3 => Ok(Self::R4),
            4 => Ok(Self::R5),
            5 => Ok(Self::R6),
            6 => Ok(Self::R7),
            7 => Ok(Self::R8),
            _ => Err(()),
        }
    }
}
impl const TryFrom<i32> for Col {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::C1),
            1 => Ok(Self::C2),
            2 => Ok(Self::C3),
            3 => Ok(Self::C4),
            4 => Ok(Self::C5),
            5 => Ok(Self::C6),
            6 => Ok(Self::C7),
            7 => Ok(Self::C8),
            _ => Err(()),
        }
    }
}
impl const From<Row> for i32 {
    fn from(value: Row) -> Self {
        match value {
            Row::R1 => 0,
            Row::R2 => 1,
            Row::R3 => 2,
            Row::R4 => 3,
            Row::R5 => 4,
            Row::R6 => 5,
            Row::R7 => 6,
            Row::R8 => 7,
        }
    }
}

impl const From<Col> for i32 {
    fn from(value: Col) -> Self {
        match value {
            Col::C1 => 0,
            Col::C2 => 1,
            Col::C3 => 2,
            Col::C4 => 3,
            Col::C5 => 4,
            Col::C6 => 5,
            Col::C7 => 6,
            Col::C8 => 7,
        }
    }
}
impl const From<Row> for usize {
    fn from(value: Row) -> Self {
        match value {
            Row::R1 => 0,
            Row::R2 => 1,
            Row::R3 => 2,
            Row::R4 => 3,
            Row::R5 => 4,
            Row::R6 => 5,
            Row::R7 => 6,
            Row::R8 => 7,
        }
    }
}

impl const From<Col> for usize {
    fn from(value: Col) -> Self {
        match value {
            Col::C1 => 0,
            Col::C2 => 1,
            Col::C3 => 2,
            Col::C4 => 3,
            Col::C5 => 4,
            Col::C6 => 5,
            Col::C7 => 6,
            Col::C8 => 7,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Square {
    pub col: Col,
    pub row: Row,
}
impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.col, self.row)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Board(pub [[Option<Piece>; ROW_COUNT]; COL_COUNT]);
impl Board {
    #[must_use]
    pub const fn new() -> Self {
        Self([[None; ROW_COUNT]; COL_COUNT])
    }

    #[allow(clippy::cast_sign_loss)]
    pub const fn place(&mut self, square: Square, piece: Piece) {
        let col = usize::from(square.col);
        let row = usize::from(square.row);
        self.0[col][row] = Some(piece);
    }
    #[must_use]
    pub const fn lookup(&self, square: Square) -> Option<Piece> {
        let col = usize::from(square.col);
        let row = usize::from(square.row);
        self.0[col][row]
    }
}
impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
impl Board {
    #[must_use]
    #[rustfmt::skip]
    pub fn filled(with_pawns: bool) -> Self {
        use PieceKind::{Pawn, Knight, Bishop, Rook, King, Queen};
        use PlayerKind::{White, Black};
        use Row::{R1, R2, R3, R4, R5, R6, R7, R8};
        use Col::{C1, C2, C3, C4, C5, C6, C7, C8};
        let mut board = Self::new();

        board.place(Square{ col: C1, row: R1 }, Piece{ kind: Rook,   owner: White });
        board.place(Square{ col: C2, row: R1 }, Piece{ kind: Knight, owner: White });
        board.place(Square{ col: C3, row: R1 }, Piece{ kind: Bishop, owner: White });
        board.place(Square{ col: C4, row: R1 }, Piece{ kind: Queen,  owner: White });
        board.place(Square{ col: C5, row: R1 }, Piece{ kind: King,   owner: White });
        board.place(Square{ col: C6, row: R1 }, Piece{ kind: Bishop, owner: White });
        board.place(Square{ col: C7, row: R1 }, Piece{ kind: Knight, owner: White });
        board.place(Square{ col: C8, row: R1 }, Piece{ kind: Rook,   owner: White });

        board.place(Square{ col: C1, row: Row::R8 }, Piece{ kind: Rook,   owner: Black });
        board.place(Square{ col: C2, row: Row::R8 }, Piece{ kind: Knight, owner: Black });
        board.place(Square{ col: C3, row: Row::R8 }, Piece{ kind: Bishop, owner: Black });
        board.place(Square{ col: C4, row: Row::R8 }, Piece{ kind: Queen,  owner: Black });
        board.place(Square{ col: C5, row: Row::R8 }, Piece{ kind: King,   owner: Black });
        board.place(Square{ col: C6, row: Row::R8 }, Piece{ kind: Bishop, owner: Black });
        board.place(Square{ col: C7, row: Row::R8 }, Piece{ kind: Knight, owner: Black });
        board.place(Square{ col: C8, row: Row::R8 }, Piece{ kind: Rook,   owner: Black });

        if with_pawns{
            for col in Col::COLS {
                board.place(Square{ col, row: R2 }, Piece { kind: Pawn, owner: White });
                board.place(Square{ col, row: R7 }, Piece { kind: Pawn, owner: Black });
            }
        }

        board
    }
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub owner: PlayerKind,
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum PlayerKind {
    White,
    Black,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Offset {
    pub col: i32,
    pub row: i32,
}
#[allow(clippy::upper_case_acronyms)]
impl Offset {
    pub const U: Self = Self { col: 1, row: 0 };
    pub const D: Self = Self { col: -1, row: 0 };
    pub const L: Self = Self { col: 0, row: -1 };
    pub const R: Self = Self { col: 0, row: 1 };
    pub const UL: Self = Self::U + Self::L;
    pub const UR: Self = Self::U + Self::R;
    pub const DL: Self = Self::D + Self::L;
    pub const DR: Self = Self::D + Self::R;
    pub const UUL: Self = Self::U + Self::UL;
    pub const UUR: Self = Self::U + Self::UR;
    pub const ULL: Self = Self::UL + Self::L;
    pub const URR: Self = Self::UR + Self::R;
    pub const DDL: Self = Self::D + Self::DL;
    pub const DDR: Self = Self::D + Self::DL;
    pub const DLL: Self = Self::DL + Self::L;
    pub const DRR: Self = Self::DR + Self::R;
    pub const UU: Self = Self::U + Self::U;
    pub const DD: Self = Self::D + Self::D;

    pub const ROOK: [Offset; 4] = [Self::U, Self::D, Self::L, Self::R];
    pub const BISHOP: [Offset; 4] = [Self::UL, Self::UR, Self::DL, Self::DR];
    pub const QUEEN: [Offset; 8] = [
        Self::U,
        Self::D,
        Self::L,
        Self::R,
        Self::UL,
        Self::UR,
        Self::DL,
        Self::DR,
    ];
    pub const KING_DIRECT: [Offset; 8] = [
        Self::U,
        Self::D,
        Self::L,
        Self::R,
        Self::UL,
        Self::UR,
        Self::DL,
        Self::DR,
    ];
    pub const PAWN_UP_SINGLE: [Offset; 1] = [Self::U];
    pub const PAWN_UP_DOUBLE: [Offset; 1] = [Self::UU];
    pub const PAWN_UP_DIAGONAL: [Offset; 2] = [Self::UL, Self::UR];

    pub const PAWN_DOWN_SINGLE: [Offset; 1] = [Self::D];
    pub const PAWN_DOWN_DOUBLE: [Offset; 1] = [Self::D];
    pub const PAWN_DOWN_DIAGONAL: [Offset; 2] = [Self::DL, Self::DR];

    pub const KNIGHT: [Offset; 8] = [
        Self::UUL,
        Self::UUR,
        Self::ULL,
        Self::URR,
        Self::DDL,
        Self::DDR,
        Self::DLL,
        Self::DRR,
    ];
}
impl const std::ops::Add for Offset {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}
impl const std::ops::Mul<i32> for Offset {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            col: self.col * rhs,
            row: self.row * rhs,
        }
    }
}

impl const std::ops::Add<Offset> for Square {
    type Output = Result<Self, ()>;
    fn add(self, rhs: Offset) -> Self::Output {
        Ok(Self {
            col: (self.col + rhs.col)?,
            row: (self.row + rhs.row)?,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum MoveKind {
    Pawn(PawnMove),
    Knight(KnightMove),
    Bishop(BishopMove),
    Rook(RookMove),
    Queen(QueenMove),
    King(KingMove),
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum PawnMove {
    Step,
    DoubleStep,
    Capture,
    EnPassant,
    StepPromote,
    CapturePromote,
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum KnightMove {
    Move,
    Capture,
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum BishopMove {
    Move,
    Capture,
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum RookMove {
    Move,
    Capture,
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum QueenMove {
    Move,
    Capture,
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display)]
pub enum KingMove {
    Move,
    Capture,
    CastleShort,
    CastleLong,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub kind: MoveKind,
    pub start: Square,
    pub end: Square,
}
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} => {}", self.kind, self.start, self.end)
    }
}
impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} => {}", self.kind, self.start, self.end)
    }
}
