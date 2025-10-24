use crate::coord::{Col, Row, Square};
use crate::game::*;
use crate::mov::Move;
use crate::piece::Piece;
use crate::player::PlayerKind;
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

    for depth in 0..=1 {
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
        println!("{col}: {}", Col::to_ascii_char(col).as_str());
    }

    println!("-------Rows--------");
    for row in Row::ROWS {
        assert_eq!(
            row,
            Row::try_from_ascii_char(Row::to_ascii_char(row)).unwrap()
        );
        println!("{row}: {}", Row::to_ascii_char(row).as_str());
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
            "{player_kind}: {}",
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
    let fens = include_str!("../fens/lichess_puzzle_fens.txt");

    for fen in fens.lines() {
        let schach_game = GameState::try_from_fen(fen).unwrap();
        let schach_fen = schach_game.to_fen();
        assert_eq!(fen, schach_fen.as_str());
    }
}
