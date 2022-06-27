use crate::square::Square;
use avocado::game::piece::Piece;
use dioxus::prelude::*;

#[derive(Props)]
pub struct BoardProps<F> {
    pub squares: [Option<Piece>; 64],
    pub click: F,
    #[props(!optional)]
    pub highlighted: Option<usize>,
}

pub fn Board<F: Fn(usize)>(cx: Scope<BoardProps<F>>) -> Element {
    cx.render(rsx!(div {
        class: "board",

        cx.props.squares.iter().enumerate().map(|(sq, &piece)| rsx!(Square {
            pos: sq,
            highlighted: Some(sq) == cx.props.highlighted,
            piece: piece,
            click: move |pos| (cx.props.click)(pos),
        }))
    }))
}
