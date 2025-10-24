use crate::{
    board::Board,
    coord::{Col, Row, Square},
    game::{CastlingRights, FullMoveCount, GameState, Position},
    piece::Piece,
    player::PlayerKind,
};
use std::ascii::Char as AsciiChar;

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
        let [
            piece_placements,
            active_player,
            castling_availability,
            en_passant_target_square,
            half_move_clock,
            full_move_number,
        ] = binding.as_slice()
        else {
            panic!()
        };

        let piece_placements_chunked: [[Option<Piece>; 8]; 8] = piece_placements
            .split(|c| *c == AsciiChar::from_u8(b'/').unwrap())
            .map(fen_row_to_board_row)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let board = Board(
            mattr::transpose_array(piece_placements_chunked).map(|mut col| {
                col.reverse();
                col
            }),
        );

        let fifty_move_rule_clock: u64 = half_move_clock.as_str().parse().unwrap();

        let white_castling_rights = CastlingRights {
            kingside: castling_availability.contains(&AsciiChar::CapitalK), // `K`
            queenside: castling_availability.contains(&AsciiChar::CapitalQ), // `Q`
        };

        let black_castling_rights = CastlingRights {
            kingside: castling_availability.contains(&AsciiChar::SmallK), // `k`
            queenside: castling_availability.contains(&AsciiChar::SmallQ), // `q`
        };

        let position_history: Vec<Position> = vec![];

        let active_player = PlayerKind::try_from(active_player.as_slice()).unwrap();

        let en_passant_target = Square::from_acs(en_passant_target_square).unwrap();

        let full_move_count = FullMoveCount(full_move_number.as_str().parse().unwrap());

        Self {
            board,
            fifty_move_rule_clock,
            white_castling_rights,
            black_castling_rights,
            position_history,
            is_perft: false,
            active_player,
            en_passant_target,
            full_move_count,
        }
    }

    #[must_use]
    pub fn to_fen(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub struct NotANumber;
impl TryFrom<&[AsciiChar]> for FullMoveCount {
    type Error = std::num::ParseIntError;

    fn try_from(value: &[AsciiChar]) -> Result<Self, Self::Error> {
        value.as_str().parse().map(Self)
    }
}
#[derive(Debug)]
pub struct NoPiece;

impl const TryFrom<AsciiChar> for Piece {
    type Error = NoPiece;
    fn try_from(value: AsciiChar) -> Result<Self, Self::Error> {
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
}

impl const From<Piece> for AsciiChar {
    fn from(value: Piece) -> Self {
        use Piece as P;
        match value {
            P::PAWN_WHITE => Self::CapitalP,   // `P`
            P::KNIGHT_WHITE => Self::CapitalN, // `N`
            P::BISHOP_WHITE => Self::CapitalB, // `B`
            P::ROOK_WHITE => Self::CapitalR,   // `R`
            P::QUEEN_WHITE => Self::CapitalQ,  // `Q`
            P::KING_WHITE => Self::CapitalK,   // `K`

            P::PAWN_BLACK => Self::SmallP,   // `p`
            P::KNIGHT_BLACK => Self::SmallN, // `n`
            P::BISHOP_BLACK => Self::SmallB, // `b`
            P::ROOK_BLACK => Self::SmallR,   // `r`
            P::QUEEN_BLACK => Self::SmallQ,  // `q`
            P::KING_BLACK => Self::SmallK,   // `k`
        }
    }
}

#[derive(Debug)]
pub struct InvalidPlayer;

impl TryFrom<&[AsciiChar]> for PlayerKind {
    type Error = InvalidPlayer;

    fn try_from(value: &[AsciiChar]) -> Result<Self, Self::Error> {
        use AsciiChar as AC;
        match value {
            [AC::SmallW] => Ok(Self::White),             // `w`
            [AC::SmallB] => Ok(Self::Black),             // `b`
            [] | [_] | [_, _, ..] => Err(InvalidPlayer), // empty, other or 2+ chars are all wrong
        }
    }
}

fn fen_row_to_board_row(row: &[AsciiChar]) -> [Option<Piece>; 8] {
    let mut out_row: Vec<Option<Piece>> = vec![];

    for c in row {
        match *c as u8 {
            d @ b'1'..=b'8' => out_row.extend(vec![None; usize::from(d - b'0')]),
            b'A'..=b'Z' | b'a'..=b'z' => out_row.push(Some(TryInto::try_into(*c).unwrap())),

            _ => panic!(),
        }
    }

    out_row
        .try_into()
        .expect("why did the row not have 8 things in it :susge:")
}

impl Square {
    pub fn from_acs(value: &[AsciiChar]) -> Result<Option<Self>, ()> {
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
}
