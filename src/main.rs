use clap::Parser;
use clearscreen;
use std::{fmt::Display, io, thread, time};

struct Game {
    board: Board,
    player_count: usize,
}

impl Game {
    // Creates a new instance of Game
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            player_count: Game::get_player_count(),
        }
    }

    fn get_player_count() -> usize {
        loop {
            let mut input = String::new();
            clearscreen::clear().unwrap();
            println!("How many players are there (1/2):");
            io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read line");
            let input = match input.trim().parse() {
                Ok(1) => 1,
                Ok(2) => 2,
                _ => {
                    clearscreen::clear().unwrap();
                    println!("That was not an option");
                    thread::sleep(time::Duration::from_millis(850));
                    continue;
                }
            };
            return input;
        }
    }
}

#[derive(Clone, Debug)]
enum Symbol {
    Circle,
    Cross,
}

struct Board {
    rows: usize,
    columns: usize,
    spaces: Vec<Vec<Option<Symbol>>>,
}

impl Board {
    /// Creates a new instance of Board
    ///
    /// # Panics
    ///
    /// If the rows or columns provided are less than 3
    /// the program will panic
    fn new() -> Board {
        let args = tic_tac_toe::Args::parse();

        assert!(args.rows >= 3 && args.columns >= 3);

        let mut spaces = Vec::with_capacity(args.rows - 1);
        for _ in 0..args.rows {
            spaces.push(vec![None; args.columns])
        }

        Board {
            rows: args.rows,
            columns: args.columns,
            spaces,
        }
    }
}

// TODO: Refactor
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        clearscreen::clear().unwrap();

        let mut output = String::new();
        let mut rows = 0;
        loop {
            let mut column = 0;
            for i in &self.spaces[rows] {
                column += 1;
                output += "  ";
                output += match i {
                    Some(Symbol::Cross) => "x",
                    Some(Symbol::Circle) => "o",
                    None => " ",
                };

                if column < self.spaces[rows].len() {
                    output += "  |";
                }
            }
            output += "\n";

            rows += 1;

            if rows == self.rows {
                break;
            }

            for _ in 1..self.columns {
                output += "-----+";
            }

            output += "-----\n";
        }
        writeln!(f, "{}", output)
    }
}

fn main() {
    let game = Game::new();
}
