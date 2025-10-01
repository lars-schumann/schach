#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![feature(const_trait_impl, const_ops)]

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
    pub col: i32,
    pub row: i32,
}
impl Coord {
    #[must_use]
    const fn add(self, rhs: Self) -> Self {
        Self {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}
impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.col, self.row)
    }
}
impl std::ops::Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::add(self, rhs)
    }
}
impl std::ops::Mul<i32> for Coord {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            col: self.col * rhs,
            row: self.row * rhs,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Board<const COL_COUNT: usize, const ROW_COUNT: usize>(
    [[Option<Piece>; ROW_COUNT]; COL_COUNT],
);
impl<const COL_COUNT: usize, const ROW_COUNT: usize> Board<COL_COUNT, ROW_COUNT> {
    #[must_use]
    const fn new() -> Self {
        Self([[None; ROW_COUNT]; COL_COUNT])
    }

    #[allow(clippy::unused_self)] // otherwise you have to Board::<COL_COUNT, ROW_COUNT>::contains() everywhere
    #[must_use]
    const fn contains(&self, Coord { col, row }: Coord) -> bool {
        if !(0 <= col && 0 <= row) {
            return false;
        }
        #[allow(clippy::cast_sign_loss)]
        let (col, row) = (col as usize, row as usize);
        if !(col < COL_COUNT && row < ROW_COUNT) {
            return false;
        }
        true
    }
    #[allow(clippy::cast_sign_loss)]
    const fn place(&mut self, coord: Coord, piece: Piece) {
        assert!(self.contains(coord));
        self.0[coord.col as usize][coord.row as usize] = Some(piece);
    }
    #[must_use]
    pub const fn inner(&self) -> &[[Option<Piece>; ROW_COUNT]; COL_COUNT] {
        &self.0
    }
    #[must_use]
    const fn lookup(&self, coord: Coord) -> Option<Piece> {
        assert!(self.contains(coord));
        #[allow(clippy::cast_sign_loss)]
        self.0[coord.col as usize][coord.row as usize]
    }
}
impl<const COL_COUNT: usize, const ROW_COUNT: usize> Default for Board<COL_COUNT, ROW_COUNT> {
    fn default() -> Self {
        Self::new()
    }
}
impl Board<8, 8> {
    #[must_use]
    #[rustfmt::skip]
    pub fn filled() -> Self {
        use PieceKind::{Pawn, Knight, Bishop, Rook, King, Queen};
        use PlayerKind::{White, Black};
        let mut board = Self::new();

        board.place(Coord{ col: 0, row: 0 }, Piece{ kind: Rook,   owner: White });
        board.place(Coord{ col: 1, row: 0 }, Piece{ kind: Knight, owner: White });
        board.place(Coord{ col: 2, row: 0 }, Piece{ kind: Bishop, owner: White });
        board.place(Coord{ col: 3, row: 0 }, Piece{ kind: Queen,  owner: White });
        board.place(Coord{ col: 4, row: 0 }, Piece{ kind: King,   owner: White });
        board.place(Coord{ col: 5, row: 0 }, Piece{ kind: Bishop, owner: White });
        board.place(Coord{ col: 6, row: 0 }, Piece{ kind: Knight, owner: White });
        board.place(Coord{ col: 7, row: 0 }, Piece{ kind: Rook,   owner: White });

        board.place(Coord{ col: 0, row: 7 }, Piece{ kind: Rook,   owner: Black });
        board.place(Coord{ col: 1, row: 7 }, Piece{ kind: Knight, owner: Black });
        board.place(Coord{ col: 2, row: 7 }, Piece{ kind: Bishop, owner: Black });
        board.place(Coord{ col: 3, row: 7 }, Piece{ kind: Queen,  owner: Black });
        board.place(Coord{ col: 4, row: 7 }, Piece{ kind: King,   owner: Black });
        board.place(Coord{ col: 5, row: 7 }, Piece{ kind: Bishop, owner: Black });
        board.place(Coord{ col: 6, row: 7 }, Piece{ kind: Knight, owner: Black });
        board.place(Coord{ col: 7, row: 7 }, Piece{ kind: Rook,   owner: Black });

        for col in 0..8 {
            board.place(Coord{ col, row: 1 }, Piece { kind: Pawn, owner: White });
            board.place(Coord{ col, row: 6 }, Piece { kind: Pawn, owner: Black });
        }

        board
    }
}

#[allow(clippy::upper_case_acronyms)]
enum Direction {
    U,
    D,
    L,
    R,
    UL,
    UR,
    DL,
    DR,
    UUL,
    UUR,
    ULL,
    URR,
    DDL,
    DDR,
    DLL,
    DRR,
    UU,
    DD,
}
impl Direction {
    #[must_use]
    const fn to_coord(&self) -> Coord {
        use Direction::{
            D, DD, DDL, DDR, DL, DLL, DR, DRR, L, R, U, UL, ULL, UR, URR, UU, UUL, UUR,
        };
        match self {
            U => Coord { col: 1, row: 0 },
            D => Coord { col: -1, row: 0 },
            L => Coord { col: 0, row: -1 },
            R => Coord { col: 0, row: 1 },
            UL => U + L,
            UR => U + R,
            DL => D + L,
            DR => D + R,
            UUL => U + UL,
            UUR => U + UR,
            ULL => UL + L,
            URR => UR + R,
            DDL => D + DL,
            DDR => D + DR,
            DLL => DL + L,
            DRR => DR + R,
            UU => U + U,
            DD => D + D,
        }
    }
    const ROOK: [Coord; 4] = [
        Self::U.to_coord(),
        Self::D.to_coord(),
        Self::L.to_coord(),
        Self::R.to_coord(),
    ];
    const BISHOP: [Coord; 4] = [
        Self::UL.to_coord(),
        Self::UR.to_coord(),
        Self::DL.to_coord(),
        Self::DR.to_coord(),
    ];
    const QUEEN: [Coord; 8] = [
        Self::U.to_coord(),
        Self::D.to_coord(),
        Self::L.to_coord(),
        Self::R.to_coord(),
        Self::UL.to_coord(),
        Self::UR.to_coord(),
        Self::DL.to_coord(),
        Self::DR.to_coord(),
    ];
    const KING_DIRECT: [Coord; 8] = [
        Self::U.to_coord(),
        Self::D.to_coord(),
        Self::L.to_coord(),
        Self::R.to_coord(),
        Self::UL.to_coord(),
        Self::UR.to_coord(),
        Self::DL.to_coord(),
        Self::DR.to_coord(),
    ];
    const PAWN_UP_SINGLE: [Coord; 1] = [Self::U.to_coord()];
    const PAWN_UP_DOUBLE: [Coord; 1] = [Self::UU.to_coord()];
    const PAWN_UP_DIAGONAL: [Coord; 2] = [Self::UL.to_coord(), Self::UR.to_coord()];

    const PAWN_DOWN_SINGLE: [Coord; 1] = [Self::D.to_coord()];
    const PAWN_DOWN_DOUBLE: [Coord; 1] = [Self::D.to_coord()];
    const PAWN_DOWN_DIAGONAL: [Coord; 2] = [Self::DL.to_coord(), Self::DR.to_coord()];

    const KNIGHT: [Coord; 8] = [
        Self::UUL.to_coord(),
        Self::UUR.to_coord(),
        Self::ULL.to_coord(),
        Self::URR.to_coord(),
        Self::DDL.to_coord(),
        Self::DDR.to_coord(),
        Self::DLL.to_coord(),
        Self::DRR.to_coord(),
    ];
}
impl const std::ops::Add for Direction {
    type Output = Coord;
    fn add(self, rhs: Self) -> Self::Output {
        self.to_coord().add(rhs.to_coord())
    }
}
fn attacked_squares<const COL_COUNT: usize, const ROW_COUNT: usize>(
    board: &Board<COL_COUNT, ROW_COUNT>,
    starting_square: Coord,
    active_player: PlayerKind,
    directions: &[Coord],
    range_upper_bound: Option<i32>,
) -> impl Iterator<Item = Coord> + use<COL_COUNT, ROW_COUNT> {
    let sqr = board.lookup(starting_square);
    assert!(sqr.is_some_and(|piece| piece.owner == active_player));

    let rays = directions.iter().map(move |direction| {
        (1..range_upper_bound.unwrap_or(i32::MAX))
            .map(move |i| starting_square + *direction * i)
            .take_while(|coord| board.contains(*coord))
    });
    let mut out = vec![];
    for ray in rays {
        for target_square in ray {
            match board.lookup(target_square) {
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
}
fn find_moves<const COL_COUNT: usize, const ROW_COUNT: usize>(
    board: &Board<COL_COUNT, ROW_COUNT>,
    starting_square: Coord,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    let Some(piece) = board.lookup(starting_square) else {
        return vec![].into_iter();
    };
    if piece.owner != active_player {
        return vec![].into_iter();
    }

    match piece.kind {
        PieceKind::Pawn => pawn_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
        PieceKind::Knight => knight_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
        PieceKind::Bishop => bishop_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
        PieceKind::Rook => rook_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
        PieceKind::Queen => queen_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
        PieceKind::King => king_moves(board, starting_square, active_player)
            .collect::<Vec<Move>>()
            .into_iter(),
    }
}

fn knight_moves<const COL_COUNT: usize, const ROW_COUNT: usize>(
    board: &Board<COL_COUNT, ROW_COUNT>,
    starting_square: Coord,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::Knight,
            owner: active_player
        })
    );
    attacked_squares(
        board,
        starting_square,
        active_player,
        &Direction::KNIGHT,
        Some(1),
    )
    .map(move |target_square| match board.lookup(target_square) {
        None => Move {
            kind: MoveKind::Knight(KnightMove::Move),
            start: starting_square,
            end: target_square,
        },
        Some(piece) if piece.owner != active_player => Move {
            kind: MoveKind::Knight(KnightMove::Capture),
            start: starting_square,
            end: target_square,
        },
        _ => unreachable!(),
    })
}

fn bishop_moves<const COL_COUNT: usize, const ROW_COUNT: usize>(
    board: &Board<COL_COUNT, ROW_COUNT>,
    starting_square: Coord,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::Bishop,
            owner: active_player
        })
    );
    attacked_squares(
        board,
        starting_square,
        active_player,
        &Direction::BISHOP,
        None,
    )
    .map(move |target_square| match board.lookup(target_square) {
        None => Move {
            kind: MoveKind::Bishop(BishopMove::Move),
            start: starting_square,
            end: target_square,
        },
        Some(piece) if piece.owner != active_player => Move {
            kind: MoveKind::Bishop(BishopMove::Capture),
            start: starting_square,
            end: target_square,
        },
        _ => unreachable!(),
    })
}

fn rook_moves<const COL_COUNT: usize, const ROW_COUNT: usize>(
    board: &Board<COL_COUNT, ROW_COUNT>,
    starting_square: Coord,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::Rook,
            owner: active_player
        })
    );
    attacked_squares(
        board,
        starting_square,
        active_player,
        &Direction::BISHOP,
        None,
    )
    .map(move |target_square| match board.lookup(target_square) {
        None => Move {
            kind: MoveKind::Rook(RookMove::Move),
            start: starting_square,
            end: target_square,
        },
        Some(piece) if piece.owner != active_player => Move {
            kind: MoveKind::Rook(RookMove::Capture),
            start: starting_square,
            end: target_square,
        },
        _ => unreachable!(),
    })
}

fn queen_moves<const COL_COUNT: usize, const ROW_COUNT: usize>(
    board: &Board<COL_COUNT, ROW_COUNT>,
    starting_square: Coord,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::Queen,
            owner: active_player
        })
    );
    attacked_squares(
        board,
        starting_square,
        active_player,
        &Direction::QUEEN,
        None,
    )
    .map(move |target_square| match board.lookup(target_square) {
        None => Move {
            kind: MoveKind::Queen(QueenMove::Move),
            start: starting_square,
            end: target_square,
        },
        Some(piece) if piece.owner != active_player => Move {
            kind: MoveKind::Queen(QueenMove::Capture),
            start: starting_square,
            end: target_square,
        },
        _ => unreachable!(),
    })
}

fn king_moves<const COL_COUNT: usize, const ROW_COUNT: usize>(
    board: &Board<COL_COUNT, ROW_COUNT>,
    starting_square: Coord,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::King,
            owner: active_player
        })
    );
    attacked_squares(
        board,
        starting_square,
        active_player,
        &Direction::KING_DIRECT,
        Some(1),
    )
    .map(move |target_square| match board.lookup(target_square) {
        None => Move {
            kind: MoveKind::King(KingMove::Move),
            start: starting_square,
            end: target_square,
        },
        Some(piece) if piece.owner != active_player => Move {
            kind: MoveKind::King(KingMove::Capture),
            start: starting_square,
            end: target_square,
        },
        _ => unreachable!(),
    })
}

fn pawn_moves<const COL_COUNT: usize, const ROW_COUNT: usize>(
    board: &Board<COL_COUNT, ROW_COUNT>,
    starting_square: Coord,
    active_player: PlayerKind,
) -> impl Iterator<Item = Move> {
    assert_eq!(
        board.lookup(starting_square),
        Some(Piece {
            kind: PieceKind::Pawn,
            owner: active_player
        })
    );
    let (forward_one, forward_two, forward_diagonal, promotion_row) = if active_player
        == PlayerKind::White
    {
        (
                Direction::PAWN_UP_SINGLE,
                Direction::PAWN_UP_DOUBLE,
                Direction::PAWN_UP_DIAGONAL,
                i32::try_from(ROW_COUNT - 1).expect("dont choose a row count of 0 bro (or if this overflowed choose a smaller board smh)"),
            )
    } else {
        (
            Direction::PAWN_DOWN_SINGLE,
            Direction::PAWN_DOWN_DOUBLE,
            Direction::PAWN_DOWN_DIAGONAL,
            0,
        )
    };

    let single_step =
        attacked_squares(board, starting_square, active_player, &forward_one, Some(1)).filter_map(
            move |target_square| match board.lookup(target_square) {
                None => Some(if target_square.row == promotion_row {
                    Move {
                        kind: MoveKind::Pawn(PawnMove::StepPromote),
                        start: starting_square,
                        end: target_square,
                    }
                } else {
                    Move {
                        kind: MoveKind::Pawn(PawnMove::Step),
                        start: starting_square,
                        end: target_square,
                    }
                }),
                Some(piece) if piece.owner != active_player => None,
                _ => unreachable!(),
            },
        );

    let double_step =
        attacked_squares(board, starting_square, active_player, &forward_two, Some(1)).filter_map(
            move |target_square| match board.lookup(target_square) {
                None => Some(Move {
                    kind: MoveKind::Pawn(PawnMove::DoubleStep),
                    start: starting_square,
                    end: target_square,
                }),
                Some(piece) if piece.owner != active_player => None,
                _ => unreachable!(),
            },
        );

    let captures = attacked_squares(
        board,
        starting_square,
        active_player,
        &forward_diagonal,
        Some(1),
    )
    .filter_map(move |target_square| match board.lookup(target_square) {
        None => None,
        Some(piece) if piece.owner != active_player => {
            if target_square.row == promotion_row {
                Some(Move {
                    kind: MoveKind::Pawn(PawnMove::CapturePromote),
                    start: starting_square,
                    end: target_square,
                })
            } else {
                Some(Move {
                    kind: MoveKind::Pawn(PawnMove::Capture),
                    start: starting_square,
                    end: target_square,
                })
            }
        }
        _ => unreachable!(),
    });

    single_step.chain(double_step).chain(captures)
}

enum MoveKind {
    Pawn(PawnMove),
    Knight(KnightMove),
    Bishop(BishopMove),
    Rook(RookMove),
    Queen(QueenMove),
    King(KingMove),
}
enum PawnMove {
    Step,
    DoubleStep,
    Capture,
    EnPassant,
    StepPromote,
    CapturePromote,
}

enum KnightMove {
    Move,
    Capture,
}

enum BishopMove {
    Move,
    Capture,
}

enum RookMove {
    Move,
    Capture,
}

enum QueenMove {
    Move,
    Capture,
}

enum KingMove {
    Move,
    Capture,
    CastleShort,
    CastleLong,
}

struct Move {
    kind: MoveKind,
    start: Coord,
    end: Coord,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    kind: PieceKind,
    owner: PlayerKind,
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
enum PlayerKind {
    White,
    Black,
}
