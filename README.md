# memg - Memory Game

`memg` is a interactive and fun to use memory game written in rust

## Installation

- Make sure you have rust installed. Use this [official installation guide](https://www.rust-lang.org/learn/get-started)
- Clone the repo: `git clone https://github.com/devhypercoder/memg.git`
- Build: `cargo bnild --release`. Compiled binary would be found in `target/release/memg` You may copy this to a directory in `$PATH`

If you wish to directly run the program, `cargo run -- <flags / options>` should do the trick

## Options

- `-b`: Board size to play on. Specify the number of rows/columns.
- `-t`: Testing. Removes the timer.

For more information, run `memg --help` / `cargo run -- --help`

## Play

```
Board
 I  M  P
 T  X  I
 W  M  G

Lives: 5
```

**Basic Info:**

- You have 5 lives to begin with.
- Rows/Columns start at `1` 

**Play:**

- Enter the value at the coordinates when asked.
- Not case-sensitive
- Once you get all correct answers, you win
- If 5 lives are taken OR the game ends without you getting every correct answer, you lose.

## Contributing

Please feel free to make a [issue](https://github.com/DevHyperCoder/memg/issues) or a [Pull Request](https://github.com/DevHyperCoder/memg/pulls)

> If you plan on adding a feature, please discuss it first by making a [issue](https://github.com/DevHyperCoder/memg/issues)

**If you like the game, consider giving it a** [star](https://github.com/DevHyperCoder/memg/stargazers) **on Github**

## LICENSE

`memg` is licensed under the GNU General Public License 3 (GPL-3.0). Our copy of the GPL-3.0 license can be found [here](./LICENSE)
