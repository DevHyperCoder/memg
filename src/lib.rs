/***
memg - A interactive and fun to use memory game written in rust
Copyright (C) 2021 DevHyperCoder

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use rand::{prelude::SliceRandom, thread_rng};
use std::fmt::Display;

const BOARD_SIZE: usize = 3;
const LIVES: usize = 5;
pub const DURATION: u32 = 6000;

pub struct Game {
    pub lives: usize,
    pub board: Vec<char>,
    pub status: GameStatus,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            lives: LIVES,
            status: GameStatus::InProgress,
            board: get_board(BOARD_SIZE),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Err(e) = writeln!(f, "Board") {
            return Err(e);
        }

        let mut row = 0;
        while row < BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if let Err(e) = write!(f, " {} ", self.board[row * BOARD_SIZE + col]) {
                    return Err(e);
                }
            }

            row += 1;
            if let Err(e) = writeln!(f) {
                return Err(e);
            }
        }
        writeln!(f, "\nLives: {}", self.lives)
    }
}

fn get_board(s: usize) -> Vec<char> {
    let mut stats = vec![];

    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();

    for _ in 0..s * s {
        let rand_letter = letters.choose(&mut thread_rng());
        stats.push(rand_letter.unwrap().to_owned().to_ascii_uppercase())
    }

    stats
}

pub enum GameStatus {
    InProgress,
    Lost,
    Won,
}
