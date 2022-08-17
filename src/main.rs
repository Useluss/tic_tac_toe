use clap::Parser;

#[derive(Clone, Debug)]
enum Symbol {
    Circle,
    Cross,
}

fn main() {
    let board = Board::new();

    println!("{}, {}", board.columns, board.rows);
    println!("{:?}", board.spaces);
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

        println!("{}, {}", args.rows, args.columns);

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
