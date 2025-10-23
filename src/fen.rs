use crate::{
    board::Board,
    coord::{Col, Row, Square},
    game::{CastlingRights, FullMoveCount, GameState, Position},
    piece::Piece,
    player::PlayerKind,
};

impl GameState {
    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        let [
            piece_placements,
            active_player,
            castling_availability,
            en_passant_target_square,
            half_move_clock,
            full_move_number,
        ] = fen
            .split_ascii_whitespace()
            .map(str::to_owned)
            .collect::<Vec<_>>()
            .as_slice()
        else {
            panic!()
        };

        let piece_placements_chunked: [[Option<Piece>; 8]; 8] = piece_placements
            .split('/')
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

        let fifty_move_rule_clock: u64 = half_move_clock.parse().unwrap();

        let white_castling_rights = CastlingRights {
            kingside: castling_availability.contains('K'),
            queenside: castling_availability.contains('Q'),
        };

        let black_castling_rights = CastlingRights {
            kingside: castling_availability.contains('k'),
            queenside: castling_availability.contains('q'),
        };

        let position_history: Vec<Position> = vec![];

        let active_player = PlayerKind::White; //PlayerKind::try_from(active_player).unwrap();

        let en_passant_target = Square::from_str(en_passant_target_square).unwrap();

        let full_move_count = FullMoveCount(6);

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
impl TryFrom<&str> for FullMoveCount {
    type Error = std::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse().map(Self)
    }
}
#[derive(Debug)]
pub struct NoPiece;

impl const TryFrom<u8> for Piece {
    type Error = NoPiece;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
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

impl const From<Piece> for &str {
    fn from(value: Piece) -> Self {
        match value {
            Piece::PAWN_WHITE => "P",
            Piece::KNIGHT_WHITE => "N",
            Piece::BISHOP_WHITE => "B",
            Piece::ROOK_WHITE => "R",
            Piece::QUEEN_WHITE => "Q",
            Piece::KING_WHITE => "K",

            Piece::PAWN_BLACK => "p",
            Piece::KNIGHT_BLACK => "n",
            Piece::BISHOP_BLACK => "b",
            Piece::ROOK_BLACK => "r",
            Piece::QUEEN_BLACK => "q",
            Piece::KING_BLACK => "k",
        }
    }
}

#[derive(Debug)]
pub struct InvalidPlayer;

impl TryFrom<&str> for PlayerKind {
    type Error = InvalidPlayer;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "w" => Ok(Self::White),
            "b" => Ok(Self::Black),
            _ => Err(InvalidPlayer),
        }
    }
}

fn fen_row_to_board_row(row: &str) -> [Option<Piece>; 8] {
    let mut out_row: Vec<Option<Piece>> = vec![];

    for c in row.bytes() {
        match c {
            d @ b'1'..=b'8' => out_row.extend(vec![None; usize::from(d - b'0')]),
            b'A'..=b'Z' | b'a'..=b'z' => out_row.push(Some(c.try_into().unwrap())),

            _ => panic!(),
        }
    }

    out_row
        .try_into()
        .expect("why did the row not have 8 things in it :susge:")
}

impl Square {
    #[must_use]
    pub const fn from_str(value: &str) -> Result<Option<Self>, ()> {
        if value == "-" {
            return Ok(None);
        }

        todo!()
    }
}
