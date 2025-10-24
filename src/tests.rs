use owlchess::board;

use crate::board::*;
use crate::coord::Square;
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

    for depth in 0..=5 {
        continued_games.clone().into_iter().for_each(|game| {
            let legal_moves: Vec<Move> = game.legal_moves().collect();
            assert_eq!(
                legal_moves.len(),
                owlchess::movegen::legal::gen_all(
                    &owlchess::Board::from_fen(game.to_fen().as_str()).unwrap()
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
fn test_fens() {
    for square in Square::all() {
        assert_eq!(
            Some(square),
            Square::try_from_fen(&Square::to_fen(&Some(square))).unwrap()
        );
        println!("{}", Square::to_fen(&Some(square)).as_str());
    }
}
