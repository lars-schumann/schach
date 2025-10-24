use crate::{
    board::Board,
    coord::{Col, Row, Square},
    game::{CastlingRights, FullMoveCount, GameState},
    piece::Piece,
    player::PlayerKind,
};
use std::ascii::Char as AsciiChar;

pub struct FenStrings {
    piece_placements: Vec<AsciiChar>,
    active_player: Vec<AsciiChar>,
    castling_availability: Vec<AsciiChar>,
    en_passant_target_square: Vec<AsciiChar>,
    half_move_clock: Vec<AsciiChar>,
    full_move_number: Vec<AsciiChar>,
}

impl GameState {
    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        let binding = fen
            .split_ascii_whitespace()
            .map(|str| {
                str.bytes()
                    .map(AsciiChar::from_u8)
                    .map(Option::unwrap)
                    .collect::<Vec<AsciiChar>>()
            })
            .collect::<Vec<_>>();

        let fen = FenStrings {
            piece_placements: binding[0].clone(),
            active_player: binding[1].clone(),
            castling_availability: binding[2].clone(),
            en_passant_target_square: binding[3].clone(),
            half_move_clock: binding[4].clone(),
            full_move_number: binding[5].clone(),
        };

        let board = Board::try_from_fen(fen.piece_placements.as_slice()).unwrap();

        let fifty_move_rule_clock: u64 = fen.half_move_clock.as_str().parse().unwrap();

        let white_castling_rights = CastlingRights {
            kingside: fen.castling_availability.contains(&AsciiChar::CapitalK), // `K`
            queenside: fen.castling_availability.contains(&AsciiChar::CapitalQ), // `Q`
        };

        let black_castling_rights = CastlingRights {
            kingside: fen.castling_availability.contains(&AsciiChar::SmallK), // `k`
            queenside: fen.castling_availability.contains(&AsciiChar::SmallQ), // `q`
        };

        let active_player = PlayerKind::try_from_fen(fen.active_player.as_slice()).unwrap();
        let en_passant_target =
            Square::try_from_fen(fen.en_passant_target_square.as_slice()).unwrap();
        let full_move_count = FullMoveCount::try_from_fen(fen.full_move_number.as_slice()).unwrap();

        Self {
            board,
            fifty_move_rule_clock,
            white_castling_rights,
            black_castling_rights,
            position_history: vec![],
            is_perft: false,
            active_player,
            en_passant_target,
            full_move_count,
        }
    }

    #[must_use]
    pub fn to_fen(&self) -> Vec<AsciiChar> {
        let Self {
            board,
            fifty_move_rule_clock,
            white_castling_rights,
            black_castling_rights,
            position_history,
            en_passant_target,
            active_player,
            is_perft,
            full_move_count,
        } = self;

        todo!()
    }
}

#[derive(Debug)]
pub struct NotANumber;
impl FullMoveCount {
    fn try_from_fen(value: &[AsciiChar]) -> Result<Self, std::num::ParseIntError> {
        value.as_str().parse().map(Self)
    }

    fn to_fen(self) -> Vec<AsciiChar> {
        self.0.to_string().as_ascii().unwrap().to_owned()
    }
}
#[derive(Debug)]
pub struct NoPiece;

impl Piece {
    const fn try_from_fen(value: AsciiChar) -> Result<Self, NoPiece> {
        match value as u8 {
            b'P' => Ok(Self::PAWN_WHITE),
            b'N' => Ok(Self::KNIGHT_WHITE),
            b'B' => Ok(Self::BISHOP_WHITE),
            b'R' => Ok(Self::ROOK_WHITE),
            b'Q' => Ok(Self::QUEEN_WHITE),
            b'K' => Ok(Self::KING_WHITE),

            b'p' => Ok(Self::PAWN_BLACK),
            b'n' => Ok(Self::KNIGHT_BLACK),
            b'b' => Ok(Self::BISHOP_BLACK),
            b'r' => Ok(Self::ROOK_BLACK),
            b'q' => Ok(Self::QUEEN_BLACK),
            b'k' => Ok(Self::KING_BLACK),

            _ => Err(NoPiece),
        }
    }

    const fn to_fen(value: Self) -> AsciiChar {
        use AsciiChar as AC;
        match value {
            Self::PAWN_WHITE => AC::CapitalP,   // `P`
            Self::KNIGHT_WHITE => AC::CapitalN, // `N`
            Self::BISHOP_WHITE => AC::CapitalB, // `B`
            Self::ROOK_WHITE => AC::CapitalR,   // `R`
            Self::QUEEN_WHITE => AC::CapitalQ,  // `Q`
            Self::KING_WHITE => AC::CapitalK,   // `K`

            Self::PAWN_BLACK => AC::SmallP,   // `p`
            Self::KNIGHT_BLACK => AC::SmallN, // `n`
            Self::BISHOP_BLACK => AC::SmallB, // `b`
            Self::ROOK_BLACK => AC::SmallR,   // `r`
            Self::QUEEN_BLACK => AC::SmallQ,  // `q`
            Self::KING_BLACK => AC::SmallK,   // `k`
        }
    }
}

#[derive(Debug)]
pub struct InvalidPlayer;

impl PlayerKind {
    fn try_from_fen(value: &[AsciiChar]) -> Result<Self, InvalidPlayer> {
        match value {
            [AsciiChar::SmallW] => Ok(Self::White),      // `w`
            [AsciiChar::SmallB] => Ok(Self::Black),      // `b`
            [] | [_] | [_, _, ..] => Err(InvalidPlayer), // empty, other or 2+ chars are all wrong
        }
    }

    const fn to_fen(self) -> AsciiChar {
        match self {
            Self::White => AsciiChar::SmallW,
            Self::Black => AsciiChar::SmallB,
        }
    }
}

fn fen_row_to_board_row(row: &[AsciiChar]) -> [Option<Piece>; 8] {
    let mut out_row: Vec<Option<Piece>> = vec![];

    for c in row {
        match *c as u8 {
            d @ b'1'..=b'8' => out_row.extend(vec![None; usize::from(d - b'0')]),
            b'A'..=b'Z' | b'a'..=b'z' => out_row.push(Some(Piece::try_from_fen(*c).unwrap())),

            _ => panic!(),
        }
    }

    out_row
        .try_into()
        .expect("why did the row not have 8 things in it :susge:")
}

impl Board {
    pub fn try_from_fen(value: &[AsciiChar]) -> Result<Self, ()> {
        let piece_placements_chunked: [[Option<Piece>; 8]; 8] = value
            .split(|c| *c == AsciiChar::from_u8(b'/').unwrap())
            .map(fen_row_to_board_row)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let board = Self(
            mattr::transpose_array(piece_placements_chunked).map(|mut col| {
                col.reverse();
                col
            }),
        );

        Ok(board)
    }
}

impl Square {
    pub fn try_from_fen(value: &[AsciiChar]) -> Result<Option<Self>, ()> {
        // supposed to look like `-` || `a5` / `d2` / `f7` / ...
        match value {
            [_] => Ok(None), // normally just `-`, but i'll be nice
            [col, row] => Ok(Some(Self {
                col: Col::try_from(*col as u8 - b'a' + 1)?,
                row: Row::try_from(*row as u8 - b'0' + 1)?,
            })),
            [] | [_, _, _, ..] => Err(()), // should never empty or longer than 2
        }
    }

    #[must_use]
    pub fn to_fen(value: Option<Self>) -> Vec<AsciiChar> {
        match value {
            None => vec![AsciiChar::Solidus], // `/`
            Some(Self { col, row }) => todo!(),
        }
    }
}
