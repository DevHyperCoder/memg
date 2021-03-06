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
use structopt::StructOpt;

pub const LIVES: usize = 5;
pub const DURATION: u32 = 6000;

#[derive(Clone)]
pub struct Game {
    pub lives: usize,
    pub board: Vec<char>,
    pub status: GameStatus,
    pub correct: usize,
    pub coordinates: Vec<(usize, usize)>,
    current_index: usize,
    board_size:usize
}

impl Game {
    pub fn new(board_size:usize) -> Self {
        Game {
            lives: LIVES,
            status: GameStatus::InProgress,
            board: get_board(board_size),
            correct: 0,
            coordinates: Game::get_shuffled_coordinate_array(board_size),
            current_index: 0,
            board_size,
        }
    }

    fn get_shuffled_coordinate_array(board_size:usize) -> Vec<(usize, usize)> {
        let mut coordinates = Game::get_coordinate_array(board_size);

        coordinates.shuffle(&mut thread_rng());

        coordinates
    }

    fn get_coordinate_array(board_size:usize) -> Vec<(usize, usize)> {
        let mut coords = vec![];

        let mut row = 0;

        while row < board_size {
            for col in 0..board_size {
                coords.push((row + 1, col + 1))
            }
            row += 1;
        }

        coords
    }

    pub fn get_status(&self) -> String {
        match self.status {
            GameStatus::Won => {
                format!("You won!\nLives remaining: {}", self.lives)
            }
            GameStatus::Lost => {
                format!(
                    "You lost!\nYou got {} correct {}",
                    self.correct,
                    self.get_guess()
                )
            }
            GameStatus::InProgress => {
                format!("Correct: {}\nLives: {}", self.correct, self.lives)
            }
        }
    }

    fn get_guess(&self) -> &str {
        if self.correct == 1 {
            return "guess";
        }

        "guesses"
    }

    pub fn check_value(&self, position: (usize, usize), value: char) -> bool {
        self.get_value(position) == value
    }

    pub fn get_value(&self, position: (usize, usize)) -> char {
        self.board[(position.0 - 1) * self.board_size + (position.1 - 1)]
    }

    pub fn get_coord(&self) -> (usize, usize) {
        self.coordinates[self.current_index]
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self.status,GameStatus::InProgress)
    }

    pub fn increment_correct(&mut self) {
        self.correct += 1;
        self.current_index += 1;

        self.calculate_game_status()
    }

    pub fn decrease_lives(&mut self) {
        self.current_index += 1;
        self.lives -= 1;

        self.calculate_game_status()
    }

    fn calculate_game_status(&mut self) {
        self.status = if self.lives == 0 {
            GameStatus::Lost
        } else if self.correct == self.board_size * self.board_size {
            GameStatus::Won
        } else if self.current_index == self.board_size * self.board_size {
            GameStatus::Lost
        } else {
            GameStatus::InProgress
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Err(e) = writeln!(f, "Board") {
            return Err(e);
        }

        let mut row = 0;
        while row < self.board_size {
            for col in 0..self.board_size {
                if let Err(e) = write!(f, " {} ", self.board[row * self.board_size + col]) {
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

#[derive(Clone)]
pub enum GameStatus {
    InProgress,
    Lost,
    Won,
}

#[derive(StructOpt)]
/// A interactive and fun to use memory game written in rust
pub struct Args {
    /// Removes timer.
    #[structopt(short,long="test")]
    pub testing:bool,

    #[structopt(short,long,default_value="3")]
    /// Board size to play on. Specify the number of rows/columns.
    pub board_size:usize
}
