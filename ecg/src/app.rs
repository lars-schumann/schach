use schach::board::Board;
use schach::coord::Square;
use schach::piece::Piece;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let board = Board::new();

    let game = schach::game::GameState::new();
    let legals = game.core.legal_moves().collect::<Vec<_>>();

    html! {
        <main class="board-root">
            <h1>{ "Every Chess Game" }</h1>
            <div class="boards">
            {for legals.iter().map(|mv| html!{<BoardDisplay board={game.core.board} from_to={FromTo { from: mv.origin, to: mv.destination }}/>})}
            </div>
        </main>
    }
}

#[derive(PartialEq)]
pub struct FromTo {
    from: Square,
    to: Square,
}
#[derive(PartialEq, Properties)]
pub struct BoardProps {
    pub board: Board,
    pub from_to: FromTo,
}

#[function_component]
pub fn BoardDisplay(props: &BoardProps) -> Html {
    let board = &props.board;
    let from_to = &props.from_to;

    html! {
        <div class="chessboard">
            {
                for Square::ALL.into_iter().map(|sq| {
                    let piece_opt = board[sq];
                    let piece_char = piece_opt.map(|p|p.to_string()).unwrap_or_default();

                    let mut class = "cell".to_string();
                    if sq == from_to.from || sq == from_to.to{
                        class.push_str(" highlight");
                    } else if sq.is_black(){
                        class.push_str(" dark");
                    } else {
                        class.push_str(" light")
                    }


                    html! {
                        <div class={class}>
                            { piece_char }
                        </div>
                    }
                })
            }
        </div>
    }
}
