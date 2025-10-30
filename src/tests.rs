use crate::coord::{Col, Row, Square};
use crate::game::*;
use crate::piece::Piece;
use crate::player::PlayerKind;

#[cfg(feature = "rayon")]
#[test]
fn search() {
    let depth = 3;
    let game = GameState::try_from_fen(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    )
    .unwrap();

    let before = std::time::Instant::now();

    let stats = game.search(depth);

    println!("---------------------------");
    println!("depth: {depth}");
    println!("#checkmate: {}", stats.checkmated_games);
    println!("#drawn games: {}", stats.drawn_games);
    println!("#continued games: {}", stats.countinued_games);
    println!("#en passant: {}", stats.en_passant);
    println!("elapsed: {:?}", before.elapsed());
    println!("---------------------------");
}

#[test]
fn test_fens() {
    println!("---Squares-------");
    for square in Square::all() {
        assert_eq!(
            Some(square),
            Square::try_from_fen(&Square::to_fen(&Some(square))).unwrap()
        );

        println!("{square:?}: {}", Square::to_fen(&Some(square)).as_str());
    }

    println!("-------Columns-----");
    for col in Col::COLS {
        assert_eq!(
            col,
            Col::try_from_ascii_char(Col::to_ascii_char(col)).unwrap()
        );
        println!("{col:?}: {}", Col::to_ascii_char(col).as_str());
    }

    println!("-------Rows--------");
    for row in Row::ROWS {
        assert_eq!(
            row,
            Row::try_from_ascii_char(Row::to_ascii_char(row)).unwrap()
        );
        println!("{row:?}: {}", Row::to_ascii_char(row).as_str());
    }

    println!("-------Pieces------");
    for piece in Piece::ALL {
        assert_eq!(
            piece,
            Piece::try_from_ascii_char(Piece::to_ascii_char(piece)).unwrap()
        );
        println!("{piece}: {}", Piece::to_ascii_char(piece).as_str());
    }

    println!("-------Players-----");
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

    println!("-------Castling-----");
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

#[test]
fn test_mass_fens() {
    let fens = std::fs::read_to_string("./fens/lichess_puzzle_fens.txt").unwrap();

    for fen in fens.lines() {
        let schach_game = GameState::try_from_fen(fen).unwrap();
        let schach_fen = schach_game.to_fen();
        assert_eq!(fen, schach_fen.as_str());
    }
}

#[cfg(feature = "rayon")]
#[test]
fn test_mass_against_owl() {
    let max_depth = 3;
    let max_fens = 10;
    let skip_fens = 100;
    let progress_thingy = std::cmp::max(max_fens / 1_000, 1);
    let fens = std::fs::read_to_string("./fens/lichess_puzzle_fens.txt").unwrap();

    let mut progress = 0;

    for fen in fens.lines().skip(skip_fens).take(max_fens) {
        let mut game = GameState::try_from_fen(fen).unwrap();
        game.is_perft = true;
        let _ = game.search(max_depth);
        progress += 1;
        if progress % progress_thingy == 0 {
            println!("{progress}/{max_fens}");
        }
    }
}
