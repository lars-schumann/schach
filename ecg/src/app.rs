use std::ops::Not;

use schach::board::Board;
use schach::coord::Square;
use schach::game::GameResult;
use schach::game::GameState;
use schach::game::StepResult;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| vec![StepResult::Continued(GameState::new())]);

    let state_n = (*state).clone();

    match state_n.last() {
        Some(StepResult::Continued(game)) => {
            let legals = {
                let game = game.clone();
                game.core.legal_moves().collect::<Vec<_>>()
            };

            let handle = state.clone();

            let undo_on_click = {
                let mut state_copy = (*state).clone();
                state_copy.pop();
                move |_| {
                    if state_copy.is_empty().not() {
                        handle.set(state_copy.clone());
                    }
                }
            };

            html! {
                <main class="board-root">
                    <h1>{ "Every Chess Game" }</h1>
                    <div class="boards">
                    <UndoButton on_click={undo_on_click}/>
                        { for legals.iter().map(|mv| {
                            let game = game.clone();
                            let mv = *mv;
                            let state = state.clone();

                            html!{
                                <BoardDisplay
                                    board={game.core.board}
                                    from_to={FromTo { from: mv.origin, to: mv.destination }}
                                    on_click={move |_| {
                                        let new_game = game.clone().step(mv);
                                        let mut state_copy = (*state).clone();
                                        state_copy.push(new_game);
                                        state.set(state_copy);
                                    }}
                                />
                            }
                        }) }
                    </div>
                </main>
            }
        }
        Some(StepResult::Terminated(GameResult { kind, .. })) => {
            html! {
                <main class="board-root">
                    <h1>{ "Every Chess Game" }</h1>
                    <div class="boards">
                        { if kind.is_win(){
                            "WIN"
                        } else {
                            "DRAW"
                        } }
                    </div>
                </main>
            }
        }
        None => panic!(),
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

#[derive(PartialEq, Properties)]
pub struct UndoButtonProps {
    pub on_click: Callback<()>,
}

#[function_component]
pub fn UndoButton(props: &UndoButtonProps) -> Html {
    let onclick = {
        let cb = props.on_click.clone();
        Callback::from(move |_| cb.emit(()))
    };

    html! { <button {onclick}>{ "UNDO" }</button> }
}
