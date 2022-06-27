use std::sync::atomic::AtomicBool;

use crate::board::Board;
use avocado::{
    engine::search::Search,
    game::{
        board::{self, GameState, UndoInfo},
        move_gen::MoveGen,
        move_list::MoveList,
    },
};
use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element {
    let board = use_state(&cx, board::Board::default);
    let gen = cx.use_hook(|_| MoveGen::default());

    let mut squares = [None; 64];
    for (i, piece) in (0..64).map(|i| (i, board[i])) {
        squares[i] = piece;
    }

    let mut moves = MoveList::default();
    gen.gen_moves(&mut moves, board);

    let selected = use_state(&cx, || None);
    let status = board.game_state(gen);
    let score = use_state(&cx, || None);

    cx.render(rsx!(div {
        id: "app",
        style { [include_str!("../index.css")] }

        div {
            class: "status",

            match status {
                GameState::Draw50 => rsx!("Fifty move draw"),
                GameState::ToPlay(color) => rsx!("{color:?} to play"),
                GameState::Winner(color) => rsx!("{color:?} has won"),
                GameState::Stalemate => rsx!("Stalemate"),
                GameState::ThreefoldRepetition => rsx!("Threefold repetition"),
                GameState::LowMaterial => rsx!("Low material"),
            },

            score.map(|score| rsx!(div {
                class: "score",

                "score: {score}"
            }))
        },

        Board {
            squares: squares,
            highlighted: *selected.get(),
            click: move |sq| {
                match *selected.get() {
                    None => selected.set(Some(sq)),
                    Some(s) if s == sq => selected.set(None),
                    Some(start) => {
                        for mv in &moves {
                            let mv_start = usize::from(mv.start());
                            let mv_end = usize::from(mv.end());

                            if mv_start == start && mv_end == sq {
                                let mut b = (*board.get()).clone();
                                b.make_move(mv, &mut UndoInfo::default());

                                let stop = AtomicBool::new(false);
                                let mut search = Search::new(b.clone(), gen, &stop, |_| ());

                                if let Some((best_move, eval)) = search.iterative_deepening_basic(6) {
                                    b.make_move(best_move, &mut UndoInfo::default());
                                    score.set(Some(eval));
                                }

                                board.set(b);

                                break;
                            }
                        }

                        selected.set(None);
                    }
                }
            },
        }
    }))
}
