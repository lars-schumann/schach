use crate::game::*;

#[test]
fn test() {
    let game = GameState::new();

    let mut terminated_games: Vec<GameState> = vec![]; //push-only 
    let mut continued_games: Vec<GameState> = vec![game]; //reset every turn
    let mut new_continued_games: Vec<GameState> = vec![];

    let mut en_passant_count = 0;

    let before = std::time::Instant::now();

    for depth in 0..4 {
        continued_games.clone().into_iter().for_each(|game| {
            let legal_moves: Vec<Move> = game.legal_moves().collect();
            for mov in legal_moves.clone() {
                if matches!(mov, Move::EnPassant { .. }) {
                    en_passant_count += 1;
                }
                match game.clone().step(mov, legal_moves.clone()) {
                    StepResult::Terminated(GameResult {
                        kind: _,
                        final_game_state,
                    }) => terminated_games.push(final_game_state),
                    StepResult::Continued(game_state) => {
                        new_continued_games.push(game_state);
                    }
                }
            }
        });
        continued_games.clone_from(&new_continued_games);
        new_continued_games.clear();

        println!("depth: {depth}");
        println!("terminated games: {}", terminated_games.len());
        println!("continued games: {}", continued_games.len());
        println!("en passant count:{}", &en_passant_count);
    }

    let after = std::time::Instant::now();
    println!("took: {:?}", after - before);
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
