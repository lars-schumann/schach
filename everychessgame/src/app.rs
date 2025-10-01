use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let board = schach::Board::filled();
    html! {
        <main class="bg-green-300">
        <div class="grid grid-cols-8">
            {
                for board.inner().iter().flat_map(|col| {
                    col.iter().map(|&piece| {
                        html! {
                            <Square
                                piece={ piece }
                                coordinates={ schach::Coord{row: 1, col:1} }
                            />
                        }
                    })
                })
            }
        </div>
        </main>
    }
}

#[derive(PartialEq, Properties)]
pub struct SquareProps {
    piece: Option<schach::Piece>,
    coordinates: schach::Coord,
}

#[function_component]
pub fn Square(props: &SquareProps) -> Html {
    let SquareProps { piece, coordinates } = props;
    let symbol = match piece {
        None => "",
        Some(piece) => &piece.to_string().clone(),
    };
    html! {
        <div>
            { symbol }
            { coordinates.to_string() }
        </div>
    }
}
