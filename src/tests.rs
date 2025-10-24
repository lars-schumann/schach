use crate::coord::Square;
use crate::game::*;
use crate::mov::Move;
use rayon::prelude::*;
use std::sync::Mutex;
use std::sync::atomic::AtomicU32;

#[test]
fn search() {
    let game = GameState::perft();

    let terminated_games_checkmate: Mutex<Vec<GameState>> = Mutex::new(vec![]); //push-only 
    let terminated_games_draw: Mutex<Vec<GameState>> = Mutex::new(vec![]); //push-only 
    let mut continued_games: Mutex<Vec<GameState>> = Mutex::new(vec![game]);
    let mut new_continued_games: Mutex<Vec<GameState>> = Mutex::new(vec![]);

    let count: AtomicU32 = AtomicU32::new(0);

    let en_passant_count = Mutex::new(0);

    let before = std::time::Instant::now();

    for depth in 0..=5 {
        continued_games
            .lock()
            .unwrap()
            .clone()
            .into_par_iter()
            .for_each(|game| {
                let count = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                if count.is_multiple_of(10_000) {
                    println!("{count}");
                }

                let legal_moves: Vec<Move> = game.legal_moves().collect();
                // assert_eq!(
                //     legal_moves.len(),
                //     owlchess::movegen::legal::gen_all(
                //         &owlchess::Board::from_fen(game.to_fen().as_str()).unwrap()
                //     )
                //     .len()
                // );
                for mov in legal_moves.clone() {
                    if matches!(mov, Move::EnPassant { .. }) {
                        *en_passant_count.lock().unwrap() += 1;
                    }
                    match game.clone().step(mov, legal_moves.clone()) {
                        StepResult::Terminated(GameResult {
                            kind: GameResultKind::Win,
                            final_game_state,
                        }) => terminated_games_checkmate
                            .lock()
                            .unwrap()
                            .push(final_game_state),
                        StepResult::Terminated(GameResult {
                            kind: GameResultKind::Draw(_),
                            final_game_state,
                        }) => terminated_games_draw.lock().unwrap().push(final_game_state),
                        StepResult::Continued(game_state) => {
                            new_continued_games.lock().unwrap().push(game_state);
                        }
                    }
                }
            });

        std::mem::swap(&mut continued_games, &mut new_continued_games);
        new_continued_games.lock().unwrap().clear();

        println!("-------------------------");
        println!("depth: {depth}");
        println!(
            "#checkmate: {}",
            terminated_games_checkmate.lock().unwrap().len()
        );
        println!(
            "#drawn games: {}",
            terminated_games_draw.lock().unwrap().len()
        );

        println!(
            "#continued games: {}",
            continued_games.lock().unwrap().len()
        );

        println!("#en passant: {}", &en_passant_count.lock().unwrap());

        println!("elapsed: {:?}", before.elapsed());
    }
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
