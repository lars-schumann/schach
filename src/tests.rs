use crate::game::*;
use std::println;

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
fn test_mass_fens() {
    let fens = std::fs::read_to_string("./fens/lichess_puzzle_fens.txt").unwrap();

    for fen in fens.lines() {
        let schach_game = GameState::try_from_fen(fen).unwrap();
        let schach_fen = schach_game.to_fen();
        assert_eq!(fen, schach_fen.as_str());
    }
}

#[test]
fn test_mass_against_owl() {
    let max_depth = 3;
    let max_fens = 10;
    let skip_fens = 100;
    let progress_thingy = core::cmp::max(max_fens / 1_000, 1);
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
