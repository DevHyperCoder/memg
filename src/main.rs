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

use std::{io::stdin, time::Duration};

use memg::{Game, BOARD_SIZE, DURATION, TESTING};
use rand::{prelude::SliceRandom, thread_rng};

fn main() {
    clear_screen();

    println!("memg - Memory Game");

    let mut game = Game::new();

    println!("{}", game);

    println!("You have {} seconds to read the board", DURATION / 1000);

    // Check lib.rs#TESTING
    // Set to False for actual gameplay
    if !TESTING {
        std::thread::sleep(Duration::from_millis(DURATION.into()));

        clear_screen();
    }

    let mut coordinates = get_coordinate_array();

    coordinates.shuffle(&mut thread_rng());

    for coord in coordinates {
        println!("Value for {:?} ?", coord);

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        user_input = user_input.trim_end().to_string();
        user_input = user_input.to_uppercase();

        let g = &mut game;

        if g.check_value(
            coord,
            user_input.chars().next().unwrap().to_ascii_uppercase(),
        ) {
            g.increment_correct();
            continue;
        }

        match g.decrease_lives() {
            None => break,
            Some(_) => {}
        }
    }

    if game.correct == BOARD_SIZE * BOARD_SIZE {
        println!("You won the game!");
        return;
    }

    println!("Bye");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_coordinate_array() -> Vec<(usize, usize)> {
    let mut coords = vec![];

    let mut row = 0;

    while row < BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            coords.push((row + 1, col + 1))
        }
        row += 1;
    }

    coords
}
