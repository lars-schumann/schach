use std::ops::Not;
pub static COL_COUNT: usize = 8;
pub static ROW_COUNT: usize = 8;
pub static REPETITIONS_TO_DRAW_COUNT: usize = 8;
pub static FIFTY_MOVE_RULE_COUNT: u64 = 50;

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
    fn all() -> impl Iterator<Item = Self> + Clone + use<> {
        Col::COLS
            .into_iter()
            .flat_map(|col| Row::ROWS.into_iter().map(move |row| Self(col, row)))
    }
}

pub struct DebugBoard {
    pub inner: Board,
    pub attacked_squares: Vec<Square>,
}
impl std::fmt::Debug for DebugBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in Row::ROWS.into_iter().rev() {
            write!(f, "{}", i32::from(row) + 1)?;
            for col in Col::COLS {
                if self.attacked_squares.contains(&Square(col, row)) {
                    write!(f, "\x1B[31m",)?;
                }
                match self.inner[Square(col, row)] {
                    None => {
                        if (i32::from(row) + i32::from(col)) % 2 == 0 {
                            write!(f, "□ ",)?;
                        } else {
                            write!(f, "■ ",)?;
                        }
                    }

                    Some(piece) => write!(f, "{piece} ")?,
                }
                if self.attacked_squares.contains(&Square(col, row)) {
                    write!(f, "\x1B[0m",)?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "  ")?;
        for col in Col::COLS {
            write!(f, "{} ", i32::from(col) + 1)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Board(pub [[Option<Piece>; ROW_COUNT]; COL_COUNT]);
impl Board {
    #[must_use]
    pub const fn new() -> Self {
        Self([[None; ROW_COUNT]; COL_COUNT])
    }

    pub fn threatening_moves_by(
        &self,
        threatened_by: PlayerKind,
    ) -> impl Iterator<Item = Threat> + Clone + use<'_> {
        Square::all().flat_map(move |square| attacked_squares(self, square, threatened_by))
    }

    pub fn threatened_squares_by(&self, threatened_by: PlayerKind) -> impl Iterator<Item = Square> {
        Square::all().flat_map(move |square| {
            attacked_squares(self, square, threatened_by)
                .into_iter()
                .map(|threat| threat.target)
        })
    }

    #[must_use]
    pub fn king_position(&self, king_owner: PlayerKind) -> Square {
        Square::all()
            .find(|square| {
                self[*square]
                    == Some(Piece {
                        kind: PieceKind::King,
                        owner: king_owner,
                    })
            })
            .expect("where did the king go?")
    }

    #[must_use]
    pub fn is_king_checked(&self, king_owner: PlayerKind) -> bool {
        self.threatened_squares_by(king_owner.opponent())
            .any(|square| square == self.king_position(king_owner))
    }

    #[allow(clippy::cast_sign_loss)]
    pub const fn movee(&mut self, start: Square, target: Square) {
        let start_col = usize::from(start.col());
        let start_row = usize::from(start.row());
        let target_col = usize::from(target.col());
        let target_row = usize::from(target.row());
        self.0[target_col][target_row] = self.0[start_col][start_row];
        self.0[start_col][start_row] = None;
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn filled(with_pawns: bool) -> Self {
        use PieceKind::{Pawn, Knight, Bishop, Rook, King, Queen};
        use PlayerKind::{White, Black};
        use Row::{R1, R2, R7, R8};
        use Col::{C1, C2, C3, C4, C5, C6, C7, C8};
        let mut board = Self::new();

        board[Square(C1, R1)] = Some(Piece{kind: Rook,   owner: White});
        board[Square(C2, R1)] = Some(Piece{kind: Knight, owner: White});
        board[Square(C3, R1)] = Some(Piece{kind: Bishop, owner: White});
        board[Square(C4, R1)] = Some(Piece{kind: Queen,  owner: White});
        board[Square(C5, R1)] = Some(Piece{kind: King,   owner: White});
        board[Square(C6, R1)] = Some(Piece{kind: Bishop, owner: White});
        board[Square(C7, R1)] = Some(Piece{kind: Knight, owner: White});
        board[Square(C8, R1)] = Some(Piece{kind: Rook,   owner: White});

        board[Square(C1, R8)] = Some(Piece{kind: Rook,   owner: Black});
        board[Square(C2, R8)] = Some(Piece{kind: Knight, owner: Black});
        board[Square(C3, R8)] = Some(Piece{kind: Bishop, owner: Black});
        board[Square(C4, R8)] = Some(Piece{kind: Queen,  owner: Black});
        board[Square(C5, R8)] = Some(Piece{kind: King,   owner: Black});
        board[Square(C6, R8)] = Some(Piece{kind: Bishop, owner: Black});
        board[Square(C7, R8)] = Some(Piece{kind: Knight, owner: Black});
        board[Square(C8, R8)] = Some(Piece{kind: Rook,   owner: Black});

        if with_pawns{
            for col in Col::COLS {
                board[Square( col,  R2 )] = Some( Piece { kind: Pawn, owner: White });
                board[Square( col,  R7 )] = Some( Piece { kind: Pawn, owner: Black });
            }
        }

        board
    }
}
impl Default for Board {
    fn default() -> Self {
        Self::filled(true)
    }
}
impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in Row::ROWS.into_iter().rev() {
            write!(f, "{}", i32::from(row) + 1)?;
            for col in Col::COLS {
                match self[Square(col, row)] {
                    None => {
                        if (i32::from(row) + i32::from(col)) % 2 == 0 {
                            write!(f, "□ ",)?;
                        } else {
                            write!(f, "■ ",)?;
                        }
                    }
                    Some(piece) => write!(f, "{piece} ")?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "  ")?;
        for col in Col::COLS {
            write!(f, "{} ", i32::from(col) + 1)?;
        }
        Ok(())
    }
}
impl const std::ops::Index<Square> for Board {
    type Output = Option<Piece>;
    fn index(&self, index: Square) -> &Self::Output {
        let col = usize::from(index.col());
        let row = usize::from(index.row());
        &self.0[col][row]
    }
}
impl const std::ops::IndexMut<Square> for Board {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        let col = usize::from(index.col());
        let row = usize::from(index.row());
        &mut self.0[col][row]
    }
}

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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display)]
pub enum PlayerKind {
    White,
    Black,
}
impl PlayerKind {
    #[must_use]
    pub const fn opponent(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    #[must_use]
    pub const fn pawn_starting_row(self) -> Row {
        match self {
            Self::White => Row::R2,
            Self::Black => Row::R7,
        }
    }

    #[must_use]
    pub const fn pawn_promotion_row(self) -> Row {
        match self {
            Self::White => Row::R8,
            Self::Black => Row::R1,
        }
    }

    #[must_use]
    pub fn castling_non_check_needed_squares(self, castling_side: CastlingSide) -> Vec<Square> {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => vec![
                Square(Col::C5, Row::R1),
                Square(Col::C6, Row::R1),
                Square(Col::C7, Row::R1),
            ],
            (Self::White, CastlingSide::Queenside) => vec![
                Square(Col::C5, Row::R1),
                Square(Col::C4, Row::R1),
                Square(Col::C3, Row::R1),
                Square(Col::C2, Row::R1),
            ],
            (Self::Black, CastlingSide::Kingside) => vec![
                Square(Col::C5, Row::R8),
                Square(Col::C6, Row::R8),
                Square(Col::C7, Row::R8),
            ],
            (Self::Black, CastlingSide::Queenside) => vec![
                Square(Col::C5, Row::R8),
                Square(Col::C4, Row::R8),
                Square(Col::C3, Row::R8),
                Square(Col::C2, Row::R8),
            ],
        }
    }

    #[must_use]
    pub const fn king_start(&self) -> Square {
        match self {
            Self::White => Square(Col::C5, Row::R1),
            Self::Black => Square(Col::C5, Row::R8),
        }
    }

    #[must_use]
    pub const fn king_castling_target(&self, castling_side: CastlingSide) -> Square {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => Square(Col::C7, Row::R1),
            (Self::White, CastlingSide::Queenside) => Square(Col::C3, Row::R1),
            (Self::Black, CastlingSide::Kingside) => Square(Col::C7, Row::R8),
            (Self::Black, CastlingSide::Queenside) => Square(Col::C3, Row::R8),
        }
    }

    #[must_use]
    pub const fn rook_start(&self, castling_side: CastlingSide) -> Square {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => Square(Col::C8, Row::R1),
            (Self::White, CastlingSide::Queenside) => Square(Col::C1, Row::R1),
            (Self::Black, CastlingSide::Kingside) => Square(Col::C1, Row::R8),
            (Self::Black, CastlingSide::Queenside) => Square(Col::C8, Row::R8),
        }
    }

    #[must_use]
    pub const fn rook_castling_target(&self, castling_side: CastlingSide) -> Square {
        match (self, castling_side) {
            (Self::White, CastlingSide::Kingside) => Square(Col::C6, Row::R1),
            (Self::White, CastlingSide::Queenside) => Square(Col::C4, Row::R1),
            (Self::Black, CastlingSide::Kingside) => Square(Col::C6, Row::R8),
            (Self::Black, CastlingSide::Queenside) => Square(Col::C4, Row::R8),
        }
    }

    #[must_use]
    pub const fn forwards_one_row(&self) -> Offset {
        match self {
            Self::White => Offset { col: 0, row: 1 },
            Self::Black => Offset { col: 0, row: -1 },
        }
    }

    #[must_use]
    pub const fn backwards_one_row(&self) -> Offset {
        match self {
            Self::White => Offset { col: 0, row: -1 },
            Self::Black => Offset { col: 0, row: 1 },
        }
    }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::Display)]
pub enum CastlingSide {
    Kingside,
    Queenside,
}

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
                write!(f, "{piece_kind:?}: {start} -> {target} {is_capture}",)
            }
            Self::DoubleStep { start, target } => write!(f, "Double Step: {start} -> {target}"),
            Self::Promotion {
                start,
                target,
                is_capture,
                replacement,
            } => write!(
                f,
                "Promotion: {start} -> {target} to: {replacement} {is_capture}",
            ),
            Self::EnPassant {
                start,
                target,
                affected_square,
            } => write!(
                f,
                "En Passant: {start} -> {target}, affected: {affected_square}",
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
impl std::fmt::Debug for Threat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} => {}", self.piece, self.start, self.target)
    }
}

pub enum DrawKind {
    Stalemate,
    ThreefoldRepetition,
    FiftyMove,
}

pub enum GameResultKind {
    Draw(DrawKind),
    Win,
}
pub struct GameResult {
    pub kind: GameResultKind,
    pub final_game_state: GameState,
}

pub enum StepResult {
    Terminated(GameResult),
    Continued(GameState),
}

#[derive(Clone, Default)]
pub struct GameState {
    pub board: Board,
    pub round_of_last_pawn_move_or_capture: u64,
    pub white_castling_rights: CastlingRights,
    pub black_castling_rights: CastlingRights,
    pub position_history: Vec<Position>,
    pub last_double_pawn_move: Option<DoublePawnMove>,
    pub round: u64,
    pub is_perft: bool,
}
impl GameState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn testing() -> Self {
        Self {
            board: Board::filled(false),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn perft() -> Self {
        Self {
            is_perft: true,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn active_player(&self) -> PlayerKind {
        match self.round % 2 {
            0 => PlayerKind::White,
            1 => PlayerKind::Black,
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub fn is_castling_allowed(&self, castling_side: CastlingSide) -> bool {
        match (self.active_player(), castling_side) {
            (PlayerKind::White, CastlingSide::Kingside) => self.white_castling_rights.kingside,
            (PlayerKind::White, CastlingSide::Queenside) => self.white_castling_rights.queenside,
            (PlayerKind::Black, CastlingSide::Kingside) => self.black_castling_rights.kingside,
            (PlayerKind::Black, CastlingSide::Queenside) => self.black_castling_rights.queenside,
        }
    }

    pub fn deny_castling(&mut self, castling_side: CastlingSide) {
        match (self.active_player(), castling_side) {
            (PlayerKind::White, CastlingSide::Kingside) => {
                self.white_castling_rights.kingside = false;
            }
            (PlayerKind::White, CastlingSide::Queenside) => {
                self.white_castling_rights.queenside = false;
            }
            (PlayerKind::Black, CastlingSide::Kingside) => {
                self.black_castling_rights.kingside = false;
            }
            (PlayerKind::Black, CastlingSide::Queenside) => {
                self.black_castling_rights.queenside = false;
            }
        }
    }

    pub fn legal_moves(&self) -> impl Iterator<Item = Move> + Clone + use<'_> {
        self.threatening_move_candidates()
            .chain(self.pawn_step_candidates())
            .chain(self.castle_candidates())
            .filter(move |mov| {
                self.clone()
                    .apply_move_to_board(*mov)
                    .board
                    .is_king_checked(self.active_player())
                    .not()
            })
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn step(self, mov: Move, all_legal_moves: Vec<Move>) -> StepResult {
        let mut new_game = self.clone().apply_move_to_board(mov);

        let current_position = Position {
            board: new_game.board,
            possible_moves: all_legal_moves,
        };

        match mov {
            Move::Normal {
                piece_kind: PieceKind::Pawn,
                start: _,
                target: _,
                is_capture: _,
            }
            | Move::DoubleStep { .. }
            | Move::Normal {
                piece_kind: _,
                start: _,
                target: _,
                is_capture: true,
            }
            | Move::Promotion { .. }
            | Move::EnPassant { .. } => {
                new_game.round_of_last_pawn_move_or_capture = self.round;
            }
            Move::Normal { .. } | Move::Castling(_) => {} //nothing
        }

        match mov {
            Move::Castling(_)
            | Move::Normal {
                piece_kind: PieceKind::King,
                start: _,
                target: _,
                is_capture: _,
            } => {
                new_game.deny_castling(CastlingSide::Kingside);
                new_game.deny_castling(CastlingSide::Queenside);
            }
            Move::Normal {
                piece_kind: PieceKind::Rook,
                start,
                target: _,
                is_capture: _,
            } => {
                for castling_side in [CastlingSide::Kingside, CastlingSide::Queenside] {
                    if start == new_game.active_player().rook_start(castling_side) {
                        new_game.deny_castling(castling_side);
                    }
                }
            }
            Move::Normal { .. }
            | Move::DoubleStep { .. }
            | Move::Promotion { .. }
            | Move::EnPassant { .. } => {} //nothing
        }

        if let Move::DoubleStep { start: _, target } = mov {
            new_game.last_double_pawn_move = Some(DoublePawnMove {
                target,
                round: new_game.round,
            });
        }

        if self.is_perft.not() {
            new_game.position_history.push(current_position.clone());

            if new_game
                .position_history
                .iter()
                .filter(|&position| *position == current_position)
                .count()
                == REPETITIONS_TO_DRAW_COUNT
            {
                return StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::ThreefoldRepetition),
                    final_game_state: new_game,
                });
            }

            if new_game.round - new_game.round_of_last_pawn_move_or_capture == FIFTY_MOVE_RULE_COUNT
            {
                return StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::FiftyMove),
                    final_game_state: new_game,
                });
            }
        }

        let mut future = new_game.clone();
        future.round += 1;
        if future.legal_moves().count() == 0 {
            return if future
                .board
                .is_king_checked(self.active_player().opponent())
            {
                StepResult::Terminated(GameResult {
                    kind: GameResultKind::Win,
                    final_game_state: new_game,
                })
            } else {
                StepResult::Terminated(GameResult {
                    kind: GameResultKind::Draw(DrawKind::Stalemate),
                    final_game_state: new_game,
                })
            };
        }

        new_game.round += 1;
        StepResult::Continued(new_game)
    }

    fn castle_candidates(&self) -> impl Iterator<Item = Move> + Clone {
        [CastlingSide::Kingside, CastlingSide::Queenside]
            .into_iter()
            .filter(|castling_side| self.is_castling_allowed(*castling_side))
            .filter(|castling_side| {
                let mut threatened_squares = self
                    .board
                    .threatened_squares_by(self.active_player().opponent());

                self.active_player()
                    .castling_non_check_needed_squares(*castling_side)
                    .iter()
                    .any(|castle_square| {
                        threatened_squares
                            .any(|threatened_square| &threatened_square == castle_square)
                    })
                    .not()
            })
            .map(Move::Castling)
    }

    fn threatening_move_candidates(&self) -> impl Iterator<Item = Move> + Clone + use<'_> {
        self.board
            .threatening_moves_by(self.active_player())
            .flat_map(|threat| self.threat_to_move_candidate(threat))
    }

    #[must_use]
    fn threat_to_move_candidate(
        &self,
        Threat {
            piece,
            start,
            target,
        }: Threat,
    ) -> Vec<Move> {
        match (piece.kind, self.board[target]) {
            (
                PieceKind::Knight
                | PieceKind::Bishop
                | PieceKind::Rook
                | PieceKind::Queen
                | PieceKind::King,
                target_square,
            ) => vec![Move::Normal {
                piece_kind: piece.kind,
                start,
                target,
                is_capture: target_square.is_some(),
            }],
            (PieceKind::Pawn, Some(_)) => {
                if target.row() == self.active_player().pawn_promotion_row() {
                    PieceKind::PROMOTION_OPTIONS
                        .iter()
                        .map(|promotion_option| Move::Promotion {
                            start,
                            target,
                            is_capture: true,
                            replacement: *promotion_option,
                        })
                        .collect()
                } else {
                    vec![Move::Normal {
                        piece_kind: piece.kind,
                        start,
                        target,
                        is_capture: true,
                    }]
                }
            }
            (PieceKind::Pawn, None) => {
                //en passant case, this is never gonna lead to promotion
                let Some(last_double_pawn_move) = self.last_double_pawn_move else {
                    return vec![];
                };
                if self.round != last_double_pawn_move.round + 1 {
                    return vec![];
                }

                let Ok(one_back_from_pawn_target) =
                    target + self.active_player().backwards_one_row()
                else {
                    panic!("this is inside the board by construction");
                };

                if one_back_from_pawn_target == last_double_pawn_move.target {
                    vec![Move::EnPassant {
                        start,
                        target,
                        affected_square: one_back_from_pawn_target,
                    }]
                } else {
                    vec![]
                }
            }
        }
    }

    #[must_use]
    fn pawn_step_candidates(&self) -> Vec<Move> {
        let mut candidates = vec![];
        for square in Square::all() {
            let player = self.active_player();
            if !(self.board[square]
                == Some(Piece {
                    kind: PieceKind::Pawn,
                    owner: player,
                }))
            {
                continue;
            }

            let one_in_front = (square + self.active_player().forwards_one_row())
                .expect("a pawn cannot exist on the last row");

            if self.board[one_in_front].is_some() {
                continue; // pawns cant capture moving forward!
            }

            if one_in_front.row() == self.active_player().pawn_promotion_row() {
                PieceKind::PROMOTION_OPTIONS
                    .iter()
                    .map(|promotion_option| Move::Promotion {
                        start: square,
                        target: one_in_front,
                        is_capture: false,
                        replacement: *promotion_option,
                    })
                    .collect_into(&mut candidates);
            } else {
                candidates.push(Move::Normal {
                    piece_kind: PieceKind::Pawn,
                    start: square,
                    target: one_in_front,
                    is_capture: false,
                });
            }

            let Ok(two_in_front) = square + self.active_player().forwards_one_row() * 2 else {
                continue; // this one can def be out of range.
            };

            if square.row() != self.active_player().pawn_starting_row() {
                continue; // pawns can only double-move when they havent moved yet!
            }

            if self.board[two_in_front].is_some() {
                continue; // pawns cant capture moving forward!
            }

            candidates.push(Move::DoubleStep {
                start: square,
                target: two_in_front,
            });
        }
        candidates
    }
    #[must_use]
    pub fn apply_move_to_board(mut self, m: Move) -> Self {
        match m {
            Move::Normal {
                piece_kind: _,
                start,
                target,
                is_capture: _,
            }
            | Move::DoubleStep { start, target } => self.board.movee(start, target),
            Move::EnPassant {
                start,
                target,
                affected_square,
            } => {
                self.board.movee(start, target);
                self.board[affected_square] = None;
            }
            Move::Promotion {
                start,
                target,
                is_capture: _,
                replacement,
            } => {
                self.board.movee(start, target);
                self.board[target] = Some(Piece {
                    kind: replacement,
                    owner: self.active_player(),
                });
            }
            Move::Castling(castling_side) => {
                self.board.movee(
                    self.active_player().king_start(),
                    self.active_player().king_castling_target(castling_side),
                );
                self.board.movee(
                    self.active_player().rook_start(castling_side),
                    self.active_player().rook_castling_target(castling_side),
                );
            }
        }
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct CastlingRights {
    pub kingside: bool,
    pub queenside: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub board: Board,
    pub possible_moves: Vec<Move>,
}

#[derive(Debug, Clone, Copy)]
pub struct DoublePawnMove {
    target: Square,
    round: u64,
}

fn attacked_squares(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> Vec<Threat> {
    let Some(piece) = board[starting_square] else {
        return vec![];
    };
    if piece.owner != active_player {
        return vec![];
    }
    let (directions, range_upper_bound) = piece.threat_directions();
    let range_upper_bound = i32::from(range_upper_bound);

    let rays = directions.iter().map(move |direction| {
        (0..range_upper_bound)
            .map(move |i| starting_square + *direction * (i + 1))
            .take_while(Result::is_ok) //}
            .map(Result::unwrap) //} FIXME: uh this cant be right
    });

    let mut out = vec![];
    for ray in rays {
        for target_square in ray {
            match board[target_square] {
                None => out.push(target_square),
                Some(piece) if piece.owner == active_player => {
                    break;
                }
                Some(piece) if piece.owner != active_player => {
                    out.push(target_square);
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
    out.into_iter()
        .map(|target_square| Threat {
            piece,
            start: starting_square,
            target: target_square,
        })
        .collect()
}
