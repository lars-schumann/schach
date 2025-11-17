use schach::board::Board;
use schach::coord::Square;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let game = use_state(schach::game::GameState::new);

    let legals = {
        let game = game.clone();
        game.core.legal_moves().collect::<Vec<_>>()
    };

    html! {
        <main class="board-root">
            <h1>{ "Every Chess Game" }</h1>
            <div class="boards">
                { for legals.iter().map(|mv| {
                    let game = game.clone();
                    let mv = *mv;

                    html!{
                        <BoardDisplay
                            board={game.core.board}
                            from_to={FromTo { from: mv.origin, to: mv.destination }}
                            on_click={Callback::from(move |_| {
                                let new_game = (*game).clone().step(mv).game_state();
                                game.set(new_game);
                            })}
                        />
                    }
                })}
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
    pub on_click: Callback<()>,
}

#[function_component]
pub fn BoardDisplay(props: &BoardProps) -> Html {
    let board = &props.board;
    let from_to = &props.from_to;

    let onclick = {
        let cb = props.on_click.clone();
        Callback::from(move |_| cb.emit(()))
    };

    html! {
        <div class="chessboard" {onclick}>
            { for Square::ALL.into_iter().map(|sq| {
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
                }) }
        </div>
    }
}
