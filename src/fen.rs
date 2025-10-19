use crate::{
    board::Board,
    coord::{Col, Row, Square},
    game::{CastlingRights, GameState, Position},
    piece::Piece,
    player::PlayerKind,
};
use std::ascii::Char as AsciiChar;

struct FenStrings {
    piece_placements: Vec<AsciiChar>,
    active_player: String,
    castling_availability: String,
    en_passant_target_square: String,
    half_move_clock: String,
    full_move_number: String,
}

impl GameState {
    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        let x = fen
            .split_ascii_whitespace()
            .map(str::to_owned)
            .collect::<Vec<String>>();
        let y: [String; 6] = x.try_into().expect("fen did not have all 6 fields");

        let z = FenStrings {
            piece_placements: str_to_vec_ascii_char(&y[0]),
            active_player: y[1].clone(),
            castling_availability: y[2].clone(),
            en_passant_target_square: y[3].clone(),
            half_move_clock: y[4].clone(),
            full_move_number: y[5].clone(),
        };

        let piece_placements_chunked: [[Option<Piece>; 8]; 8] = z
            .piece_placements
            .split(|c| *c == AsciiChar::Solidus)
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

        let fifty_move_rule_clock: u64 = z.half_move_clock.parse().unwrap();

        let white_castling_rights = CastlingRights {
            kingside: z.castling_availability.contains('K'),
            queenside: z.castling_availability.contains('Q'),
        };

        let black_castling_rights = CastlingRights {
            kingside: z.castling_availability.contains('k'),
            queenside: z.castling_availability.contains('q'),
        };

        let position_history: Vec<Position> = vec![];

        let active_player = match z.active_player.as_str() {
            "w" => PlayerKind::White,
            "b" => PlayerKind::Black,
            _ => panic!(),
        };

        let en_passant_target = str_to_square(&z.en_passant_target_square);

        Self {
            board,
            fifty_move_rule_clock,
            white_castling_rights,
            black_castling_rights,
            position_history,
            is_perft: false,
            active_player,
            en_passant_target,
        }
    }
}

fn str_to_vec_ascii_char(str: &str) -> Vec<std::ascii::Char> {
    str.bytes()
        .map(std::ascii::Char::from_u8)
        .map(Option::unwrap)
        .collect()
}

fn ascii_char_to_piece(char: AsciiChar) -> Piece {
    match char as u8 {
        b'P' => Piece::PAWN_WHITE,
        b'N' => Piece::KNIGHT_WHITE,
        b'B' => Piece::BISHOP_WHITE,
        b'R' => Piece::ROOK_WHITE,
        b'Q' => Piece::QUEEN_WHITE,
        b'K' => Piece::KING_WHITE,

        b'p' => Piece::PAWN_BLACK,
        b'n' => Piece::KNIGHT_BLACK,
        b'b' => Piece::BISHOP_BLACK,
        b'r' => Piece::ROOK_BLACK,
        b'q' => Piece::QUEEN_BLACK,
        b'k' => Piece::KING_BLACK,

        _ => panic!(),
    }
}
fn fen_row_to_board_row(row: &[AsciiChar]) -> [Option<Piece>; 8] {
    let mut out_row: Vec<Option<Piece>> = vec![];

    for c in row {
        match *c as u8 {
            d @ b'1'..=b'8' => out_row.extend(vec![None; usize::from(d - b'0')]),
            b'A'..=b'Z' | b'a'..=b'z' => out_row.push(Some(ascii_char_to_piece(*c))),

            _ => panic!(),
        }
    }

    out_row
        .try_into()
        .expect("why did the row not have 8 things in it :susge:")
}

fn str_to_square(str: &str) -> Option<Square> {
    if str.contains('-') {
        return None;
    }

    let square: [AsciiChar; 2] = str_to_vec_ascii_char(str)
        .try_into()
        .expect("square isnt 2 chars long hmm");

    let col = match square[0] as u8 {
        b'a' => Col::C1,
        b'b' => Col::C2,
        b'c' => Col::C3,
        b'd' => Col::C4,
        b'e' => Col::C5,
        b'f' => Col::C6,
        b'g' => Col::C7,
        b'h' => Col::C8,
        _ => panic!(),
    };

    let row = match square[1] as u8 {
        b'1' => Row::R1,
        b'2' => Row::R2,
        b'3' => Row::R3,
        b'4' => Row::R4,
        b'5' => Row::R5,
        b'6' => Row::R6,
        b'7' => Row::R7,
        b'8' => Row::R8,
        _ => panic!(),
    };

    Some(Square { col, row })
}
