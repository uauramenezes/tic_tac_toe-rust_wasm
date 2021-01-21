mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct TicTacToe {
    board: Vec<String>,
    winner: String,
    mark: String,
    turn: i32,
    win: bool,
}

#[wasm_bindgen]
impl TicTacToe {
    pub fn new() -> TicTacToe {
        console_error_panic_hook::set_once();
        let board = (1..10).map(|_| {
            String::new()
        }).collect();
        let winner = String::new();
        let mark = String::new();
        let win = false;
        let turn = 0;

        TicTacToe {board, winner, mark, turn, win}
    }

    pub fn play(&mut self, index: usize) -> String {
        if self.board[index] == String::from("")
        && self.turn <= 9 && !self.win{
            if self.turn % 2 == 0 {
                self.board[index] = String::from("X");
                self.mark = String::from("X");
            } else {
                self.board[index] = String::from("O");
                self.mark = String::from("O");
            }
            self.turn += 1;
        }

        let win_condition = [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];

        for arr in win_condition.iter() {
            let (a, b, c) = arr;
            if self.board[*a] == self.board[*b]
            && self.board[*a] == self.board[*c]
            && self.board[*b] == self.board[*c]
            && self.board[*a] != String::from("") {
                if self.board[*a] == "X" {
                    self.winner = String::from("Player 1 Win!");
                } else {
                    self.winner = String::from("Player 2 Win!");
                }
                self.win = true
            } else if self.turn == 9 {
                self.winner = String::from("It's a Draw!");
            }
        }

        self.mark.clone()
    }

    pub fn change_text(&self) -> String {
        if self.win || self.turn == 9 {
            self.winner.clone()
        } else if self.turn % 2 == 0{
            String::from("Player 1 turn!")
        } else {
            String::from("Player 2 turn!")
        }
    }
}
