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

use std::time::Duration;

use memg::{Game, DURATION};

fn main() {
    clear_screen();

    println!("memg - Memory Game");

    let game = Game::new();

    println!("{}", game);

    println!("You have {} seconds to read the board", DURATION / 1000);

    std::thread::sleep(Duration::from_millis(DURATION.into()));

    clear_screen();
    println!("Bye");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
