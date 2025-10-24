use owlchess::board;

use crate::board::*;
use crate::game::*;
use crate::mov::Move;

#[test]
fn search() {
    let game = GameState::perft();

    let mut terminated_games_checkmate: Vec<GameState> = vec![]; //push-only 
    let mut terminated_games_draw: Vec<GameState> = vec![]; //push-only 
    let mut continued_games: Vec<GameState> = vec![game]; //reset every turn
    let mut new_continued_games: Vec<GameState> = vec![];

    let mut en_passant_count = 0;

    let before = std::time::Instant::now();

    for depth in 0..3 {
        continued_games.clone().into_iter().for_each(|game| {
            let legal_moves: Vec<Move> = game.legal_moves().collect();
            assert_eq!(
                legal_moves.len(),
                owlchess::movegen::legal::gen_all(
                    &owlchess::Board::from_fen(dbg!(game.to_fen().as_str())).unwrap()
                )
                .len()
            );
            for mov in legal_moves.clone() {
                if matches!(mov, Move::EnPassant { .. }) {
                    en_passant_count += 1;
                }
                match game.clone().step(mov, legal_moves.clone()) {
                    StepResult::Terminated(GameResult {
                        kind: GameResultKind::Win,
                        final_game_state,
                    }) => terminated_games_checkmate.push(final_game_state),
                    StepResult::Terminated(GameResult {
                        kind: GameResultKind::Draw(_),
                        final_game_state,
                    }) => terminated_games_draw.push(final_game_state),
                    StepResult::Continued(game_state) => {
                        new_continued_games.push(game_state);
                    }
                }
            }
        });

        std::mem::swap(&mut continued_games, &mut new_continued_games);
        new_continued_games.clear();

        println!("-------------------------");
        println!("depth: {depth}");
        println!("#checkmate: {}", terminated_games_checkmate.len());
        println!("#drawn games: {}", terminated_games_draw.len());

        println!("#continued games: {}", continued_games.len());
        println!("#en passant: {}", &en_passant_count);
    }
    println!("-------------------------");
    let after = std::time::Instant::now();
    println!("took: {:?}", after - before);
    println!("-------------------------");
}

#[test]
fn test_fen() {
    let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = GameState::from_fen(default_fen);
    assert_eq!(game.board, Board::default());
    println!("{:?}", game.board);

    let schach_fen = GameState::to_fen(&game);
    assert_eq!(default_fen, schach_fen.as_str());
}

#[test]
fn test_against_owl() {
    let fens = include_str!("../fens/lichess_puzzle_fens.txt");
    for (i, fen) in fens.lines().enumerate() {
        print!("{i}:");
        let owl_board = owlchess::Board::from_fen(fen).unwrap();

        let owl_legals = owlchess::movegen::legal::gen_all(&owl_board);

        let x = owl_board.as_fen();

        let schach_game = GameState::from_fen(fen);

        #[allow(clippy::needless_collect)]
        let schach_legals = schach_game.legal_moves().collect::<Vec<_>>();

        let (owl_len, schach_len) = (owl_legals.len(), schach_legals.len());
        assert_eq!(owl_len, schach_len);

        println!(" owl: {owl_len}, schach: {schach_len}");
    }
}
