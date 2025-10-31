use crate::board::{COL_COUNT, ROW_COUNT};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Square {
    pub col: Col,
    pub row: Row,
}

impl Square {
    const fn new(col: Col, row: Row) -> Self {
        Self { col, row }
    }
    pub const A1: Self = Self::new(Col::C1, Row::R1);
    pub const A2: Self = Self::new(Col::C1, Row::R2);
    pub const A3: Self = Self::new(Col::C1, Row::R3);
    pub const A4: Self = Self::new(Col::C1, Row::R4);
    pub const A5: Self = Self::new(Col::C1, Row::R5);
    pub const A6: Self = Self::new(Col::C1, Row::R6);
    pub const A7: Self = Self::new(Col::C1, Row::R7);
    pub const A8: Self = Self::new(Col::C1, Row::R8);

    pub const B1: Self = Self::new(Col::C2, Row::R1);
    pub const B2: Self = Self::new(Col::C2, Row::R2);
    pub const B3: Self = Self::new(Col::C2, Row::R3);
    pub const B4: Self = Self::new(Col::C2, Row::R4);
    pub const B5: Self = Self::new(Col::C2, Row::R5);
    pub const B6: Self = Self::new(Col::C2, Row::R6);
    pub const B7: Self = Self::new(Col::C2, Row::R7);
    pub const B8: Self = Self::new(Col::C2, Row::R8);

    pub const C1: Self = Self::new(Col::C3, Row::R1);
    pub const C2: Self = Self::new(Col::C3, Row::R2);
    pub const C3: Self = Self::new(Col::C3, Row::R3);
    pub const C4: Self = Self::new(Col::C3, Row::R4);
    pub const C5: Self = Self::new(Col::C3, Row::R5);
    pub const C6: Self = Self::new(Col::C3, Row::R6);
    pub const C7: Self = Self::new(Col::C3, Row::R7);
    pub const C8: Self = Self::new(Col::C3, Row::R8);

    pub const D1: Self = Self::new(Col::C4, Row::R1);
    pub const D2: Self = Self::new(Col::C4, Row::R2);
    pub const D3: Self = Self::new(Col::C4, Row::R3);
    pub const D4: Self = Self::new(Col::C4, Row::R4);
    pub const D5: Self = Self::new(Col::C4, Row::R5);
    pub const D6: Self = Self::new(Col::C4, Row::R6);
    pub const D7: Self = Self::new(Col::C4, Row::R7);
    pub const D8: Self = Self::new(Col::C4, Row::R8);

    pub const E1: Self = Self::new(Col::C5, Row::R1);
    pub const E2: Self = Self::new(Col::C5, Row::R2);
    pub const E3: Self = Self::new(Col::C5, Row::R3);
    pub const E4: Self = Self::new(Col::C5, Row::R4);
    pub const E5: Self = Self::new(Col::C5, Row::R5);
    pub const E6: Self = Self::new(Col::C5, Row::R6);
    pub const E7: Self = Self::new(Col::C5, Row::R7);
    pub const E8: Self = Self::new(Col::C5, Row::R8);

    pub const F1: Self = Self::new(Col::C6, Row::R1);
    pub const F2: Self = Self::new(Col::C6, Row::R2);
    pub const F3: Self = Self::new(Col::C6, Row::R3);
    pub const F4: Self = Self::new(Col::C6, Row::R4);
    pub const F5: Self = Self::new(Col::C6, Row::R5);
    pub const F6: Self = Self::new(Col::C6, Row::R6);
    pub const F7: Self = Self::new(Col::C6, Row::R7);
    pub const F8: Self = Self::new(Col::C6, Row::R8);

    pub const G1: Self = Self::new(Col::C7, Row::R1);
    pub const G2: Self = Self::new(Col::C7, Row::R2);
    pub const G3: Self = Self::new(Col::C7, Row::R3);
    pub const G4: Self = Self::new(Col::C7, Row::R4);
    pub const G5: Self = Self::new(Col::C7, Row::R5);
    pub const G6: Self = Self::new(Col::C7, Row::R6);
    pub const G7: Self = Self::new(Col::C7, Row::R7);
    pub const G8: Self = Self::new(Col::C7, Row::R8);

    pub const H1: Self = Self::new(Col::C8, Row::R1);
    pub const H2: Self = Self::new(Col::C8, Row::R2);
    pub const H3: Self = Self::new(Col::C8, Row::R3);
    pub const H4: Self = Self::new(Col::C8, Row::R4);
    pub const H5: Self = Self::new(Col::C8, Row::R5);
    pub const H6: Self = Self::new(Col::C8, Row::R6);
    pub const H7: Self = Self::new(Col::C8, Row::R7);
    pub const H8: Self = Self::new(Col::C8, Row::R8);
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}", self.col, self.row)
    }
}
impl Square {
    pub fn all() -> impl Iterator<Item = Self> + Clone + use<> {
        Col::COLS
            .into_iter()
            .flat_map(|col| Row::ROWS.into_iter().map(move |row| Self { col, row }))
    }

    pub fn all_fen_ordered() -> impl Iterator<Item = Self> + Clone + use<> {
        Row::ROWS
            .into_iter()
            .rev()
            .flat_map(|row| Col::COLS.into_iter().map(move |col| Self { col, row }))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    type Output = Result<Self, RowIndexOutOfRange>;
    fn add(self, rhs: i32) -> Self::Output {
        let row_number: i32 = self.into();
        (row_number + rhs).try_into()
    }
}
impl const std::ops::Add<i32> for Col {
    type Output = Result<Self, ColIndexOutOfRange>;
    fn add(self, rhs: i32) -> Self::Output {
        let column_number: i32 = self.into();
        (column_number + rhs).try_into()
    }
}

macro_rules! col_into_int_impl {
    ($($ty:ty)*) => {
        $(
            impl const From<Col> for $ty {
                fn from(value: Col) -> $ty {
                    match value {
                        Col::C1 => 1,
                        Col::C2 => 2,
                        Col::C3 => 3,
                        Col::C4 => 4,
                        Col::C5 => 5,
                        Col::C6 => 6,
                        Col::C7 => 7,
                        Col::C8 => 8,
                    }
                }
            }
        )*
    }
}

macro_rules! row_into_int_impl {
    ($($ty:ty)*) => {
        $(
            impl const From<Row> for $ty {
                fn from(value: Row) -> $ty {
                    match value {
                        Row::R1 => 1,
                        Row::R2 => 2,
                        Row::R3 => 3,
                        Row::R4 => 4,
                        Row::R5 => 5,
                        Row::R6 => 6,
                        Row::R7 => 7,
                        Row::R8 => 8,
                    }
                }
            }
        )*
    }
}

#[derive(Debug)]
pub enum ColIndexOutOfRange {
    TooLow,
    TooHigh,
}

#[derive(Debug)]
pub enum RowIndexOutOfRange {
    TooLow,
    TooHigh,
}

macro_rules! col_try_from_int_impl {
    ($($ty:ty)*) => {
        $(
            impl const TryFrom<$ty> for Col {
                type Error = ColIndexOutOfRange;
                fn try_from(value: $ty) -> Result<Self, Self::Error> {
                    match value {
                        ..=0 => Err(ColIndexOutOfRange::TooLow),
                         1 => Ok(Self::C1),
                         2 => Ok(Self::C2),
                         3 => Ok(Self::C3),
                         4 => Ok(Self::C4),
                         5 => Ok(Self::C5),
                         6 => Ok(Self::C6),
                         7 => Ok(Self::C7),
                         8 => Ok(Self::C8),
                         9.. =>  Err(ColIndexOutOfRange::TooHigh),

                    }
                }
            }
        )*
    }
}

macro_rules! row_try_from_int_impl {
    ($($ty:ty)*) => {
        $(
            impl const TryFrom<$ty> for Row {
                type Error = RowIndexOutOfRange;
                fn try_from(value: $ty) -> Result<Self, Self::Error> {
                    match value {
                        ..=0 =>  Err(RowIndexOutOfRange::TooLow),
                         1 => Ok(Self::R1),
                         2 => Ok(Self::R2),
                         3 => Ok(Self::R3),
                         4 => Ok(Self::R4),
                         5 => Ok(Self::R5),
                         6 => Ok(Self::R6),
                         7 => Ok(Self::R7),
                         8 => Ok(Self::R8),
                         9.. =>  Err(RowIndexOutOfRange::TooHigh),
                    }
                }
            }
        )*
    }
}

col_into_int_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
row_into_int_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

col_try_from_int_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
row_try_from_int_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    pub col: i32,
    pub row: i32,
}
#[allow(clippy::upper_case_acronyms)]
impl Offset {
    pub const U: Self = Self { col: 0, row: 1 };
    pub const R: Self = Self { col: 1, row: 0 };
    pub const D: Self = Self::U * -1;
    pub const L: Self = Self::R * -1;
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

#[derive(Debug)]
pub enum SquareOutOfRange {
    Col(ColIndexOutOfRange),
    Row(RowIndexOutOfRange),
}
impl const From<ColIndexOutOfRange> for SquareOutOfRange {
    fn from(value: ColIndexOutOfRange) -> Self {
        Self::Col(value)
    }
}
impl const From<RowIndexOutOfRange> for SquareOutOfRange {
    fn from(value: RowIndexOutOfRange) -> Self {
        Self::Row(value)
    }
}
impl const std::ops::Add<Offset> for Square {
    type Output = Result<Self, SquareOutOfRange>;
    fn add(self, rhs: Offset) -> Self::Output {
        Ok(Self {
            col: (self.col + rhs.col)?,
            row: (self.row + rhs.row)?,
        })
    }
}
