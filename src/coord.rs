use crate::board::{COL_COUNT, ROW_COUNT};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Square(pub Col, pub Row);
impl Square {
    #[must_use]
    pub const fn col(&self) -> Col {
        self.0
    }
    #[must_use]
    pub const fn row(&self) -> Row {
        self.1
    }
}
impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.col(), self.row())
    }
}
impl Square {
    pub fn all() -> impl Iterator<Item = Self> + Clone + use<> {
        Col::COLS
            .into_iter()
            .flat_map(|col| Row::ROWS.into_iter().map(move |row| Self(col, row)))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    pub col: i32,
    pub row: i32,
}
#[allow(clippy::upper_case_acronyms)]
impl Offset {
    pub const U: Self = Self { col: 0, row: 1 };
    pub const D: Self = Self { col: 0, row: -1 };
    pub const L: Self = Self { col: -1, row: 0 };
    pub const R: Self = Self { col: 1, row: 0 };
    pub const UL: Self = Self::U + Self::L;
    pub const UR: Self = Self::U + Self::R;
    pub const DL: Self = Self::D + Self::L;
    pub const DR: Self = Self::D + Self::R;
    pub const UUL: Self = Self::U + Self::UL;
    pub const UUR: Self = Self::U + Self::UR;
    pub const ULL: Self = Self::UL + Self::L;
    pub const URR: Self = Self::UR + Self::R;
    pub const DDL: Self = Self::D + Self::DL;
    pub const DDR: Self = Self::D + Self::DR;
    pub const DLL: Self = Self::DL + Self::L;
    pub const DRR: Self = Self::DR + Self::R;
    pub const UU: Self = Self::U + Self::U;
    pub const DD: Self = Self::D + Self::D;

    pub const ROOK: [Self; 4] = [Self::U, Self::D, Self::L, Self::R];
    pub const BISHOP: [Self; 4] = [Self::UL, Self::UR, Self::DL, Self::DR];
    pub const QUEEN: [Self; 8] = [
        Self::U,
        Self::D,
        Self::L,
        Self::R,
        Self::UL,
        Self::UR,
        Self::DL,
        Self::DR,
    ];
    pub const KING_DIRECT: [Self; 8] = [
        Self::U,
        Self::D,
        Self::L,
        Self::R,
        Self::UL,
        Self::UR,
        Self::DL,
        Self::DR,
    ];
    pub const PAWN_UP_SINGLE: [Self; 1] = [Self::U];
    pub const PAWN_UP_DOUBLE: [Self; 1] = [Self::UU];
    pub const PAWN_UP_DIAGONAL: [Self; 2] = [Self::UL, Self::UR];

    pub const PAWN_DOWN_SINGLE: [Self; 1] = [Self::D];
    pub const PAWN_DOWN_DOUBLE: [Self; 1] = [Self::D];
    pub const PAWN_DOWN_DIAGONAL: [Self; 2] = [Self::DL, Self::DR];

    pub const KNIGHT: [Self; 8] = [
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
        Ok(Self((self.col() + rhs.col)?, (self.row() + rhs.row)?))
    }
}
