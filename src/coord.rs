use crate::board::{COL_COUNT, ROW_COUNT};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Square {
    pub col: Col,
    pub row: Row,
}

macro_rules! square {
    ($square_col:tt $square_row:tt) => {
        paste::paste! {
            pub const [<$square_col $square_row>]: Self = Self{col: match Col::try_from(letter_to_number!($square_col)){
                Ok(val) => val,
                Err(_) => panic!("illegal Column Index")
            }, row: match Row::try_from($square_row){
                Ok(val) => val,
                Err(_) => panic!("illegal Row Index")
            }};
        }
    };
}

#[rustfmt::skip]
macro_rules! letter_to_number {
    (A) => { 1 };
    (B) => { 2 };
    (C) => { 3 };
    (D) => { 4 };
    (E) => { 5 };
    (F) => { 6 };
    (G) => { 7 };
    (H) => { 8 };
}

impl Square {
    square!(A 1);
    square!(B 1);
    square!(C 1);
    square!(D 1);
    square!(E 1);
    square!(F 1);
    square!(G 1);
    square!(H 1);

    square!(A 2);
    square!(B 2);
    square!(C 2);
    square!(D 2);
    square!(E 2);
    square!(F 2);
    square!(G 2);
    square!(H 2);

    square!(A 3);
    square!(B 3);
    square!(C 3);
    square!(D 3);
    square!(E 3);
    square!(F 3);
    square!(G 3);
    square!(H 3);

    square!(A 4);
    square!(B 4);
    square!(C 4);
    square!(D 4);
    square!(E 4);
    square!(F 4);
    square!(G 4);
    square!(H 4);

    square!(A 5);
    square!(B 5);
    square!(C 5);
    square!(D 5);
    square!(E 5);
    square!(F 5);
    square!(G 5);
    square!(H 5);

    square!(A 6);
    square!(B 6);
    square!(C 6);
    square!(D 6);
    square!(E 6);
    square!(F 6);
    square!(G 6);
    square!(H 6);

    square!(A 7);
    square!(B 7);
    square!(C 7);
    square!(D 7);
    square!(E 7);
    square!(F 7);
    square!(G 7);
    square!(H 7);

    square!(A 8);
    square!(B 8);
    square!(C 8);
    square!(D 8);
    square!(E 8);
    square!(F 8);
    square!(G 8);
    square!(H 8);
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.col, self.row)
    }
}
impl Square {
    pub fn all() -> impl Iterator<Item = Self> + Clone + use<> {
        Col::COLS
            .into_iter()
            .flat_map(|col| Row::ROWS.into_iter().map(move |row| Self { col, row }))
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

macro_rules! col_try_from_int_impl {
    ($($ty:ty)*) => {
        $(
            impl const TryFrom<$ty> for Col {
                type Error = ();
                fn try_from(value: $ty) -> Result<Self, Self::Error> {
                    match value {
                         1 => Ok(Self::C1),
                         2 => Ok(Self::C2),
                         3 => Ok(Self::C3),
                         4 => Ok(Self::C4),
                         5 => Ok(Self::C5),
                         6 => Ok(Self::C6),
                         7 => Ok(Self::C7),
                         8 => Ok(Self::C8),
                         _ => Err(()),
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
                type Error = ();
                fn try_from(value: $ty) -> Result<Self, Self::Error> {
                    match value {
                         1 => Ok(Self::R1),
                         2 => Ok(Self::R2),
                         3 => Ok(Self::R3),
                         4 => Ok(Self::R4),
                         5 => Ok(Self::R5),
                         6 => Ok(Self::R6),
                         7 => Ok(Self::R7),
                         8 => Ok(Self::R8),
                         _ => Err(()),
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

impl const std::ops::Add<Offset> for Square {
    type Output = Result<Self, ()>;
    fn add(self, rhs: Offset) -> Self::Output {
        Ok(Self {
            col: (self.col + rhs.col)?,
            row: (self.row + rhs.row)?,
        })
    }
}
