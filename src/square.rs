use avocado::game::{color::Color, piece::Piece};
use dioxus::prelude::*;

#[derive(Props)]
pub struct SquareProps<F> {
    pub pos: usize,
    pub highlighted: bool,
    #[props(!optional)]
    pub piece: Option<Piece>,
    pub click: F,
}

pub fn Square<F: Fn(usize)>(cx: Scope<SquareProps<F>>) -> Element {
    let rank = cx.props.pos / 8;
    let file = cx.props.pos % 8;
    let color = match (rank + file) % 2 {
        0 => " white",
        _ => " black",
    };

    let highlight = match cx.props.highlighted {
        true => " is-highlighted",
        false => "",
    };

    let piece = cx.props.piece.map(|piece| match piece {
        Piece::WhitePawn => "♙",
        Piece::WhiteKnight => "♘",
        Piece::WhiteBishop => "♗",
        Piece::WhiteRook => "♖",
        Piece::WhiteQueen => "♕",
        Piece::WhiteKing => "♔",
        Piece::BlackPawn => "♟",
        Piece::BlackKnight => "♞",
        Piece::BlackBishop => "♝",
        Piece::BlackRook => "♜",
        Piece::BlackQueen => "♛",
        Piece::BlackKing => "♚",
    });
    let piece_color = cx
        .props
        .piece
        .map(|piece| match piece.color() {
            Color::White => " white-piece",
            Color::Black => " black-piece",
        })
        .unwrap_or("");

    cx.render(rsx!(div {
        class: "square{color}{highlight}",
        onclick: move |_| (cx.props.click)(cx.props.pos),

        piece.map(|piece| rsx!(div {
            class: "piece{piece_color}",
            "{piece}"
        })),
    }))
}
