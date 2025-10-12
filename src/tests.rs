use crate::game::*;
use rayon::prelude::*;
use std::sync::Mutex;

#[test]
fn test() {
    let game = GameState::new();

    let terminated_games: Mutex<Vec<GameState>> = Mutex::new(vec![]); //push-only 
    let mut continued_games: Mutex<Vec<GameState>> = Mutex::new(vec![game]); //reset every turn
    let new_continued_games: Mutex<Vec<GameState>> = Mutex::new(vec![]);

    let en_passant_count = Mutex::new(0);

    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();

    for i in 0..4 {
        continued_games
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .for_each(|game| {
                let legal_moves: Vec<Move> = game.legal_moves().collect();
                for mov in legal_moves.clone() {
                    if matches!(mov, Move::EnPassant { .. }) {
                        *en_passant_count.lock().unwrap() += 1;
                    }
                    match game.clone().step(mov, legal_moves.clone()) {
                        StepResult::Terminated(GameResult {
                            kind: _,
                            final_game_state,
                        }) => terminated_games.lock().unwrap().push(final_game_state),
                        StepResult::Continued(game_state) => {
                            new_continued_games.lock().unwrap().push(game_state);
                        }
                    }
                }
            });
        continued_games
            .lock()
            .unwrap()
            .clone_from(&new_continued_games.lock().unwrap());

        new_continued_games.lock().unwrap().clear();

        dbg!(format!("{i}: "));
        dbg!(terminated_games.lock().unwrap().len());
        dbg!(continued_games.lock().unwrap().len());
        dbg!(&en_passant_count);
    }
}
fn test_piece(piece: Piece, starting_square: Square, active_player: PlayerKind) {
    let mut board = Board::new();
    board[starting_square] = Some(piece);
    for row in Row::ROWS {
        board[Square(Col::C6, row)] = Some(Piece {
            kind: PieceKind::Pawn,
            owner: PlayerKind::Black,
        });
    }

    let mut attacking_moves: Vec<Threat> = vec![];

    for col in Col::COLS {
        for row in Row::ROWS {
            let square = Square(col, row);
            if (board[square]).is_some_and(|piece| piece.owner == active_player) {
                attacking_moves.append(&mut attacked_squares(&board, square, active_player));
            }
        }
    }

    let dbg_board = DebugBoard {
        inner: board,
        attacked_squares: attacking_moves.iter().map(|threat| threat.target).collect(),
    };
    dbg!(attacking_moves);
    dbg!(dbg_board);
}

fn attacked_squares(
    board: &Board,
    starting_square: Square,
    active_player: PlayerKind,
) -> Vec<Threat> {
    let Some(piece) = board[starting_square] else {
        return vec![];
    };
    if piece.owner != active_player {
        return vec![];
    }
    let (directions, range_upper_bound) = piece.threat_directions();
    let range_upper_bound = i32::from(range_upper_bound);

    let rays = directions.iter().map(move |direction| {
        (0..range_upper_bound)
            .map(move |i| starting_square + *direction * (i + 1))
            .take_while(Result::is_ok) //}
            .map(Result::unwrap) //} FIXME: uh this cant be right
    });

    let mut out = vec![];
    for ray in rays {
        for target_square in ray {
            match board[target_square] {
                None => out.push(target_square),
                Some(piece) if piece.owner == active_player => {
                    break;
                }
                Some(piece) if piece.owner != active_player => {
                    out.push(target_square);
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
    out.into_iter()
        .map(|target_square| Threat {
            piece,
            start: starting_square,
            target: target_square,
        })
        .collect()
}
