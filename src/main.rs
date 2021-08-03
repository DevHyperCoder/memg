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
use memg::{Args, DURATION, Game};
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();
    clear_screen();

    println!("memg - Memory Game");

    let mut game = Game::new(args.board_size);

    println!("{}", game);

    // Check lib.rs#TESTING
    // Set to False for actual gameplay
    if !args.testing {
        println!("You have {} seconds to read the board", DURATION / 1000);
        std::thread::sleep(Duration::from_millis(DURATION.into()));
        clear_screen();
    }

    loop {
        let g = &mut game;

        if !g.is_in_progress() {
            break;
        }

        let coord = g.get_coord();
        println!("Value for {:?} ?", coord);

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        user_input = user_input.trim_end().to_string();
        user_input = user_input.to_uppercase();

        if g.check_value(
            coord,
            user_input.chars().next().unwrap().to_ascii_uppercase(),
        ) {
            g.increment_correct();
        } else {
            g.decrease_lives();
        }
        println!("{}", g.get_status())
    }

    println!("Bye");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
