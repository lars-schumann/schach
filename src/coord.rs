use core::ops::Not;

use self::Col::C1;
use self::Col::C2;
use self::Col::C3;
use self::Col::C4;
use self::Col::C5;
use self::Col::C6;
use self::Col::C7;
use self::Col::C8;
use self::Row::R1;
use self::Row::R2;
use self::Row::R3;
use self::Row::R4;
use self::Row::R5;
use self::Row::R6;
use self::Row::R7;
use self::Row::R8;
use self::Square as S;
use crate::board::COL_COUNT;
use crate::board::ROW_COUNT;

#[derive_const(PartialEq, Eq)]
#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub col: Col,
    pub row: Row,
}

#[allow(clippy::use_self)]
impl Square {
    #[must_use]
    pub const fn new(col: Col, row: Row) -> Self {
        Self { col, row }
    }
    #[must_use]
    pub const fn is_black(self) -> bool {
        (u8::from(self.col) + u8::from(self.row)).is_multiple_of(2)
    }
    #[must_use]
    pub const fn is_white(self) -> bool {
        self.is_black().not()
    }

    const fn n(col: Col, row: Row) -> Self {
        Self::new(col, row)
    }
    pub const A1: S = S::n(C1, R1);
    pub const A2: S = S::n(C1, R2);
    pub const A3: S = S::n(C1, R3);
    pub const A4: S = S::n(C1, R4);
    pub const A5: S = S::n(C1, R5);
    pub const A6: S = S::n(C1, R6);
    pub const A7: S = S::n(C1, R7);
    pub const A8: S = S::n(C1, R8);

    pub const B1: S = S::n(C2, R1);
    pub const B2: S = S::n(C2, R2);
    pub const B3: S = S::n(C2, R3);
    pub const B4: S = S::n(C2, R4);
    pub const B5: S = S::n(C2, R5);
    pub const B6: S = S::n(C2, R6);
    pub const B7: S = S::n(C2, R7);
    pub const B8: S = S::n(C2, R8);

    pub const C1: S = S::n(C3, R1);
    pub const C2: S = S::n(C3, R2);
    pub const C3: S = S::n(C3, R3);
    pub const C4: S = S::n(C3, R4);
    pub const C5: S = S::n(C3, R5);
    pub const C6: S = S::n(C3, R6);
    pub const C7: S = S::n(C3, R7);
    pub const C8: S = S::n(C3, R8);

    pub const D1: S = S::n(C4, R1);
    pub const D2: S = S::n(C4, R2);
    pub const D3: S = S::n(C4, R3);
    pub const D4: S = S::n(C4, R4);
    pub const D5: S = S::n(C4, R5);
    pub const D6: S = S::n(C4, R6);
    pub const D7: S = S::n(C4, R7);
    pub const D8: S = S::n(C4, R8);

    pub const E1: S = S::n(C5, R1);
    pub const E2: S = S::n(C5, R2);
    pub const E3: S = S::n(C5, R3);
    pub const E4: S = S::n(C5, R4);
    pub const E5: S = S::n(C5, R5);
    pub const E6: S = S::n(C5, R6);
    pub const E7: S = S::n(C5, R7);
    pub const E8: S = S::n(C5, R8);

    pub const F1: S = S::n(C6, R1);
    pub const F2: S = S::n(C6, R2);
    pub const F3: S = S::n(C6, R3);
    pub const F4: S = S::n(C6, R4);
    pub const F5: S = S::n(C6, R5);
    pub const F6: S = S::n(C6, R6);
    pub const F7: S = S::n(C6, R7);
    pub const F8: S = S::n(C6, R8);

    pub const G1: S = S::n(C7, R1);
    pub const G2: S = S::n(C7, R2);
    pub const G3: S = S::n(C7, R3);
    pub const G4: S = S::n(C7, R4);
    pub const G5: S = S::n(C7, R5);
    pub const G6: S = S::n(C7, R6);
    pub const G7: S = S::n(C7, R7);
    pub const G8: S = S::n(C7, R8);

    pub const H1: S = S::n(C8, R1);
    pub const H2: S = S::n(C8, R2);
    pub const H3: S = S::n(C8, R3);
    pub const H4: S = S::n(C8, R4);
    pub const H5: S = S::n(C8, R5);
    pub const H6: S = S::n(C8, R6);
    pub const H7: S = S::n(C8, R7);
    pub const H8: S = S::n(C8, R8);

    #[rustfmt::skip]
    pub const ALL: [Self; 64] = [
        S::A8, S::B8, S::C8, S::D8, S::E8, S::F8, S::G8, S::H8,
        S::A7, S::B7, S::C7, S::D7, S::E7, S::F7, S::G7, S::H7,
        S::A6, S::B6, S::C6, S::D6, S::E6, S::F6, S::G6, S::H6,
        S::A5, S::B5, S::C5, S::D5, S::E5, S::F5, S::G5, S::H5,
        S::A4, S::B4, S::C4, S::D4, S::E4, S::F4, S::G4, S::H4,
        S::A3, S::B3, S::C3, S::D3, S::E3, S::F3, S::G3, S::H3,
        S::A2, S::B2, S::C2, S::D2, S::E2, S::F2, S::G2, S::H2,
        S::A1, S::B1, S::C1, S::D1, S::E1, S::F1, S::G1, S::H1,
    ];
}

#[derive_const(PartialEq, Eq)]
#[derive(Debug, Copy, Clone)]
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
    pub const ALL: [Self; COL_COUNT] = [
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

#[derive_const(PartialEq, Eq)]
#[derive(Debug, Copy, Clone)]
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
    pub const ALL: [Self; ROW_COUNT] = [
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
impl const core::ops::Add<i32> for Row {
    type Output = Result<Self, RowIndexOutOfRange>;
    fn add(self, rhs: i32) -> Self::Output {
        let row_number: i32 = self.into();
        (row_number + rhs).try_into()
    }
}
impl const core::ops::Add<i32> for Col {
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
    pub(crate) const U: Self = Self { col: 0, row: 1 };
    pub(crate) const D: Self = Self::U * -1;
    const R: Self = Self { col: 1, row: 0 };
    const L: Self = Self::R * -1;
    const UL: Self = Self::U + Self::L;
    const UR: Self = Self::U + Self::R;
    const DL: Self = Self::D + Self::L;
    const DR: Self = Self::D + Self::R;
    const UUL: Self = Self::U + Self::UL;
    const UUR: Self = Self::U + Self::UR;
    const ULL: Self = Self::UL + Self::L;
    const URR: Self = Self::UR + Self::R;
    const DDL: Self = Self::D + Self::DL;
    const DDR: Self = Self::D + Self::DR;
    const DLL: Self = Self::DL + Self::L;
    const DRR: Self = Self::DR + Self::R;

    pub(crate) const ROOK: [Self; 4] = [Self::U, Self::D, Self::L, Self::R];
    pub(crate) const BISHOP: [Self; 4] = [Self::UL, Self::UR, Self::DL, Self::DR];
    pub(crate) const QUEEN: [Self; 8] = [
        Self::U,
        Self::D,
        Self::L,
        Self::R,
        Self::UL,
        Self::UR,
        Self::DL,
        Self::DR,
    ];
    pub(crate) const KING_DIRECT: [Self; 8] = [
        Self::U,
        Self::D,
        Self::L,
        Self::R,
        Self::UL,
        Self::UR,
        Self::DL,
        Self::DR,
    ];

    pub(crate) const PAWN_UP_DIAGONAL: [Self; 2] = [Self::UL, Self::UR];
    pub(crate) const PAWN_DOWN_DIAGONAL: [Self; 2] = [Self::DL, Self::DR];

    pub(crate) const KNIGHT: [Self; 8] = [
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

impl const core::ops::Add for Offset {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}
impl const core::ops::Mul<i32> for Offset {
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
impl const core::ops::Add<Offset> for Square {
    type Output = Result<Self, SquareOutOfRange>;
    fn add(self, rhs: Offset) -> Self::Output {
        Ok(Self {
            col: (self.col + rhs.col)?,
            row: (self.row + rhs.row)?,
        })
    }
}
