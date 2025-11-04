use crate::board::Board;
use crate::coord::Col;
use crate::coord::ColIndexOutOfRange;
use crate::coord::Row;
use crate::coord::RowIndexOutOfRange;
use crate::coord::Square;
use crate::coord::SquareOutOfRange;
use crate::game::CastlingRights;
use crate::game::FiftyMoveRuleClock;
use crate::game::FullMoveCount;
use crate::game::GameState;
use crate::piece::Piece;
use crate::player::PlayerKind;
use alloc::borrow::ToOwned;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::ascii::Char as AsciiChar;

struct FenStrings {
    piece_placements: Vec<AsciiChar>,
    active_player: Vec<AsciiChar>,
    castling_availability: Vec<AsciiChar>,
    en_passant_target_square: Vec<AsciiChar>,
    half_move_clock: Vec<AsciiChar>,
    full_move_number: Vec<AsciiChar>,
}

#[derive(Debug)]
pub enum GameFromFenError {
    NotAscii,
    WrongFieldCount,
}
impl GameState {
    pub fn try_from_fen(fen: &str) -> Result<Self, GameFromFenError> {
        let fen_parts: [Vec<AsciiChar>; 6] = fen
            .split_ascii_whitespace()
            .map(|str| {
                str.bytes()
                    .map(AsciiChar::from_u8)
                    .collect::<Option<Vec<AsciiChar>>>()
            })
            .collect::<Option<Vec<Vec<AsciiChar>>>>()
            .ok_or(GameFromFenError::NotAscii)?
            .try_into()
            .map_err(|_| GameFromFenError::WrongFieldCount)?;

        let fen = FenStrings {
            piece_placements: fen_parts[0].clone(),
            active_player: fen_parts[1].clone(),
            castling_availability: fen_parts[2].clone(),
            en_passant_target_square: fen_parts[3].clone(),
            half_move_clock: fen_parts[4].clone(),
            full_move_number: fen_parts[5].clone(),
        };

        let board = Board::try_from_fen(fen.piece_placements.as_slice()).unwrap();

        let fifty_move_rule_clock =
            FiftyMoveRuleClock::try_from_fen(fen.half_move_clock.as_slice()).unwrap();

        let castling_rights = CastlingRights::from_fen(&fen.castling_availability);

        let active_player = PlayerKind::try_from_fen(fen.active_player.as_slice()).unwrap();
        let en_passant_target =
            Square::try_from_fen(fen.en_passant_target_square.as_slice()).unwrap();
        let full_move_count = FullMoveCount::try_from_fen(fen.full_move_number.as_slice()).unwrap();

        Ok(Self {
            board,
            fifty_move_rule_clock,
            castling_rights,
            position_history: vec![],
            is_perft: false,
            active_player,
            en_passant_target,
            full_move_count,
        })
    }

    #[must_use]
    pub fn to_fen(&self) -> Vec<AsciiChar> {
        use AsciiChar::Space;
        let Self {
            board,
            fifty_move_rule_clock,
            castling_rights,
            position_history: _, // not part of fen
            en_passant_target,
            active_player,
            is_perft: _, // not part of fen
            full_move_count,
        } = self;

        let fen = FenStrings {
            piece_placements: Board::to_fen(board),
            active_player: vec![PlayerKind::to_ascii_char(*active_player)],
            castling_availability: CastlingRights::to_fen(*castling_rights),
            en_passant_target_square: Square::option_to_fen(*en_passant_target),
            half_move_clock: FiftyMoveRuleClock::to_fen(*fifty_move_rule_clock),
            full_move_number: FullMoveCount::to_fen(*full_move_count),
        };

        [
            fen.piece_placements,
            vec![Space],
            fen.active_player,
            vec![Space],
            fen.castling_availability,
            vec![Space],
            fen.en_passant_target_square,
            vec![Space],
            fen.half_move_clock,
            vec![Space],
            fen.full_move_number,
        ]
        .concat()
    }
}

impl CastlingRights {
    #[must_use]
    fn from_fen(value: &[AsciiChar]) -> Self {
        if value == [AsciiChar::HyphenMinus] {
            return Self::default();
        }
        Self {
            white_kingside: value.contains(&AsciiChar::CapitalK), // `K`
            white_queenside: value.contains(&AsciiChar::CapitalQ), // `Q`
            black_kingside: value.contains(&AsciiChar::SmallK),   // `k`
            black_queenside: value.contains(&AsciiChar::SmallQ),  // `q`
        }
    }

    #[must_use]
    fn to_fen(self) -> Vec<AsciiChar> {
        if self == Self::default() {
            return vec![AsciiChar::HyphenMinus];
        }
        let mut out = vec![];
        if self.white_kingside {
            out.push(AsciiChar::CapitalK);
        }
        if self.white_queenside {
            out.push(AsciiChar::CapitalQ);
        }
        if self.black_kingside {
            out.push(AsciiChar::SmallK);
        }
        if self.black_queenside {
            out.push(AsciiChar::SmallQ);
        }
        assert!(!out.is_empty());
        out
    }
}

impl FullMoveCount {
    fn try_from_fen(value: &[AsciiChar]) -> Result<Self, core::num::ParseIntError> {
        value.as_str().parse().map(Self)
    }

    fn to_fen(self) -> Vec<AsciiChar> {
        self.0.to_string().as_ascii().unwrap().to_owned()
    }
}

impl FiftyMoveRuleClock {
    fn try_from_fen(value: &[AsciiChar]) -> Result<Self, core::num::ParseIntError> {
        Ok(Self::new(value.as_str().parse()?))
    }

    fn to_fen(self) -> Vec<AsciiChar> {
        self.0.to_string().as_ascii().unwrap().to_owned()
    }
}
#[derive(Debug)]
pub struct MalformedPieceError;

impl Piece {
    pub(super) const fn try_from_ascii_char(value: AsciiChar) -> Result<Self, MalformedPieceError> {
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

            _ => Err(MalformedPieceError),
        }
    }

    #[must_use]
    pub(super) const fn to_ascii_char(self) -> AsciiChar {
        use AsciiChar as AC;
        match self {
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
pub enum InvalidPlayer {
    IllegalCharacter(AsciiChar),
    TooShort(usize),
    TooLong(usize),
}

impl PlayerKind {
    const fn try_from_fen(value: &[AsciiChar]) -> Result<Self, InvalidPlayer> {
        match value {
            [AsciiChar::SmallW] => Ok(Self::White), // `w`
            [AsciiChar::SmallB] => Ok(Self::Black), // `b`
            [] => Err(InvalidPlayer::TooShort(value.len())),
            [illegal] => Err(InvalidPlayer::IllegalCharacter(*illegal)),
            [_, _, ..] => Err(InvalidPlayer::TooLong(value.len())),
        }
    }

    #[must_use]
    const fn to_ascii_char(self) -> AsciiChar {
        match self {
            Self::White => AsciiChar::SmallW,
            Self::Black => AsciiChar::SmallB,
        }
    }
}

#[derive(Debug)]
pub enum BoardFromFenError {
    IllegalCharacter(AsciiChar),
    IllegalRowDimensions,
    IllegalColDimensions,
}

impl Board {
    fn try_from_fen(value: &[AsciiChar]) -> Result<Self, BoardFromFenError> {
        fn fen_row_to_board_row(
            row: &[AsciiChar],
        ) -> Result<[Option<Piece>; 8], BoardFromFenError> {
            let mut out_row: Vec<Option<Piece>> = vec![];

            for c in row {
                match *c as u8 {
                    b'1'..=b'8' => out_row.extend(vec![None; usize::from(u8::from(*c) - b'0')]),
                    b'P' | b'N' | b'B' | b'R' | b'Q' | b'K' | b'p' | b'n' | b'b' | b'r' | b'q'
                    | b'k' => {
                        out_row.push(Some(Piece::try_from_ascii_char(*c).unwrap()));
                    }

                    _ => return Err(BoardFromFenError::IllegalCharacter(*c)),
                }
            }

            out_row
                .try_into()
                .map_err(|_| BoardFromFenError::IllegalRowDimensions)
        }

        let piece_placements_chunked: Self = value
            .split(|c| *c == AsciiChar::Solidus)
            .map(fen_row_to_board_row)
            .collect::<Result<Vec<[Option<Piece>; 8]>, BoardFromFenError>>()?
            .try_into()
            .map(Board)
            .map_err(|_| BoardFromFenError::IllegalColDimensions)?;

        let mut new_board = Self::empty();

        //TODO: un-fuck this
        for row in Row::ROWS {
            for col in Col::COLS {
                new_board[Square {
                    col: Col::try_from(u8::from(row)).unwrap(),
                    row: Row::try_from(9 - u8::from(col)).unwrap(),
                }] = piece_placements_chunked[Square { col, row }];
            }
        }

        Ok(new_board)
    }

    #[must_use]
    fn to_fen(&self) -> Vec<AsciiChar> {
        let mut running_square_count: u32 = 0;
        let mut out: Vec<AsciiChar> = vec![];
        for square in Square::all() {
            match self[square] {
                None => running_square_count += 1,
                Some(piece) => {
                    if running_square_count != 0 {
                        out.extend(
                            running_square_count
                                .to_string()
                                .as_ascii()
                                .unwrap()
                                .to_owned(),
                        ); //as the running count should never exceed 8, this should always be a single digit
                    }
                    running_square_count = 0;
                    out.push(Piece::to_ascii_char(piece));
                }
            }
            if square.col == Col::C8 {
                if running_square_count != 0 {
                    out.extend(
                        running_square_count
                            .to_string()
                            .as_ascii()
                            .unwrap()
                            .to_owned(),
                    ); //as the running count should never exceed 8, this should always be a single digit
                }
                running_square_count = 0;
                if square != Square::H1 {
                    out.push(AsciiChar::Solidus);
                }
            }
        }
        out
    }
}

impl Col {
    const fn try_from_ascii_char(value: AsciiChar) -> Result<Self, ColIndexOutOfRange> {
        Self::try_from(u8::from(value) - b'a' + 1)
    }
    #[must_use]
    const fn to_ascii_char(self) -> AsciiChar {
        AsciiChar::from_u8(u8::from(self) + b'a' - 1).unwrap()
    }
}
impl Row {
    const fn try_from_ascii_char(value: AsciiChar) -> Result<Self, RowIndexOutOfRange> {
        Self::try_from(u8::from(value) - b'0')
    }
    #[must_use]
    const fn to_ascii_char(self) -> AsciiChar {
        AsciiChar::from_u8(u8::from(self) + b'0').unwrap()
    }
}

#[derive(Debug)]
pub enum SquareFromFenError {
    OutOfRange(SquareOutOfRange),
    IllegalCharacter(AsciiChar),
    TooShort(usize),
    TooLong(usize),
}
impl From<SquareOutOfRange> for SquareFromFenError {
    fn from(value: SquareOutOfRange) -> Self {
        Self::OutOfRange(value)
    }
}
impl Square {
    fn try_from_fen(value: &[AsciiChar]) -> Result<Option<Self>, SquareFromFenError> {
        // supposed to look like `-` || `a5` / `d2` / `f7` / ...
        match value {
            [AsciiChar::HyphenMinus] => Ok(None), //  `-`
            [col, row] => Ok(Some(Self {
                col: Col::try_from_ascii_char(*col).map_err(SquareOutOfRange::from)?,
                row: Row::try_from_ascii_char(*row).map_err(SquareOutOfRange::from)?,
            })),
            [] => Err(SquareFromFenError::TooShort(value.len())),
            [illegal] => Err(SquareFromFenError::IllegalCharacter(*illegal)),
            [_, _, _, ..] => Err(SquareFromFenError::TooLong(value.len())),
        }
    }

    #[must_use]
    pub(super) fn to_fen(self) -> Vec<AsciiChar> {
        vec![Col::to_ascii_char(self.col), Row::to_ascii_char(self.row)]
    }

    #[must_use]
    fn option_to_fen(value: Option<Self>) -> Vec<AsciiChar> {
        #[allow(clippy::option_if_let_else)]
        match value {
            None => vec![AsciiChar::HyphenMinus], // `-`
            Some(square) => square.to_fen(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn test_squares() {
        for square in Square::all() {
            assert_eq!(
                Some(square),
                Square::try_from_fen(&Square::to_fen(square)).unwrap()
            );

            println!("{square:?}: {}", Square::to_fen(square).as_str());
        }
    }

    #[test]
    fn test_columns() {
        for col in Col::COLS {
            assert_eq!(
                col,
                Col::try_from_ascii_char(Col::to_ascii_char(col)).unwrap()
            );
            println!("{col:?}: {}", Col::to_ascii_char(col).as_str());
        }
    }

    #[test]
    fn test_rows() {
        for row in Row::ROWS {
            assert_eq!(
                row,
                Row::try_from_ascii_char(Row::to_ascii_char(row)).unwrap()
            );
            println!("{row:?}: {}", Row::to_ascii_char(row).as_str());
        }
    }

    #[test]
    fn test_pieces() {
        for piece in Piece::ALL {
            assert_eq!(
                piece,
                Piece::try_from_ascii_char(Piece::to_ascii_char(piece)).unwrap()
            );
            println!("{piece}: {}", Piece::to_ascii_char(piece).as_str());
        }
    }

    #[test]
    fn test_player_kinds() {
        for player_kind in [PlayerKind::White, PlayerKind::Black] {
            assert_eq!(
                player_kind,
                PlayerKind::try_from_fen(vec![PlayerKind::to_ascii_char(player_kind)].as_slice())
                    .unwrap()
            );
            println!(
                "{player_kind:?}: {}",
                PlayerKind::to_ascii_char(player_kind).as_str()
            );
        }
    }

    #[test]
    fn test_castling_rights() {
        for castling_rights in CastlingRights::all() {
            assert_eq!(
                castling_rights,
                CastlingRights::from_fen(CastlingRights::to_fen(castling_rights).as_slice())
            );
            println!(
                "{castling_rights:?}: {}",
                CastlingRights::to_fen(castling_rights).as_str()
            );
        }
    }
}
