use clap::Parser;
use clearscreen;
use std::fmt::Display;

#[derive(Clone, Debug)]
enum Symbol {
    Circle,
    Cross,
}

fn main() {
    let board = Board::new();
    println!("{}", board); // Debug line
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
