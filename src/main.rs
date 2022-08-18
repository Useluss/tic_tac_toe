use clap::Parser;
use clearscreen;
use std::{io, thread, time};

#[derive(PartialEq)]
enum Winner {
    Cross,
    Circle,
    Draw,
    None,
}

struct Game {
    board: Board,
    player_count: usize,
    player1: Symbol,
    player2: Symbol,
    player2_is_bot: bool,
    winner: Winner,
}

impl Game {
    // Creates a new instance of Game
    pub fn new() -> Game {
        let player_count = Game::get_player_count();

        let mut player2_is_bot = false;
        if player_count > 1 {
            player2_is_bot = true;
        }

        Game {
            board: Board::new(),
            player_count,
            player1: Symbol::Cross,
            player2: Symbol::Circle,
            player2_is_bot,
            winner: Winner::None,
        }
    }

    pub fn run(mut self) {
        clearscreen::clear().unwrap();
        let mut turn = self.player1;
        let mut turn_count = 0;
        while self.winner == Winner::None {
            turn_count += 1;
            self.board.display();
            Game::place_symbol(&mut self.board, turn);

            if turn == self.player1 {
                turn = self.player2;
            } else {
                turn = self.player1;
            }

            // Start checking for win at minimum amount of moves needed to win
            if turn_count >= (self.board.rows * 2) - 1 {
                self.winner = Game::check_for_win(&self.board, turn_count);
            }
        }
        self.board.display();
    }

    fn check_for_win(board: &Board, turn_count: usize) -> Winner {
        let mut line_total = 0;
        for i in &board.spaces[board.columns] {
            match i {
                Some(Symbol::Cross) => line_total += 5,
                Some(Symbol::Circle) => line_total += 1,
                None => {
                    line_total = 0;
                    continue;
                }
            }

            if line_total == 15 {
                return Winner::Cross;
            }

            if line_total == 3 {
                return Winner::Circle;
            }
        }

        if turn_count == (board.rows * board.columns) {
            return Winner::Draw;
        }
        Winner::None
    }

    fn place_symbol(board: &mut Board, turn: Symbol) {
        loop {
            let mut input = String::new();
            println!("Enter row number & column number:");
            io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read line");
            input.retain(|c| !c.is_whitespace());
            let input: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    Game::not_an_option();
                    board.display();
                    continue;
                }
            };

            let digits = Game::convert_to_digits(input);

            if digits[0] < 1 || (digits[1] as usize) > board.rows {
                Game::not_an_option();
                board.display();
                continue;
            }

            board.spaces[(digits[0] as usize) - 1][(digits[1] as usize) - 1] = Some(turn.clone());
            break;
        }
    }

    fn convert_to_digits(input: usize) -> Vec<u32> {
        let digits: Vec<u32> = input
            .to_string()
            .chars()
            .map(|num| num.to_digit(10).unwrap())
            .collect();
        digits
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
                    Game::not_an_option();
                    continue;
                }
            };
            return input;
        }
    }

    fn not_an_option() {
        clearscreen::clear().unwrap();
        println!("That was not an option");
        thread::sleep(time::Duration::from_millis(850)); // Sleep to give user time to read message
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
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
    /// or more than 9 the program will panic
    fn new() -> Board {
        let args = tic_tac_toe::Args::parse();

        assert!(args.rows_and_columns >= 3 && args.rows_and_columns <= 9);

        let (rows, columns) = (args.rows_and_columns, args.rows_and_columns);

        let mut spaces = Vec::with_capacity(rows - 1);
        for _ in 0..rows {
            spaces.push(vec![None; columns])
        }

        Board {
            rows,
            columns,
            spaces,
        }
    }

    /// Displays the game board to the screen
    pub fn display(&self) {
        clearscreen::clear().unwrap();
        let mut rows = 0;

        // Display column numbers
        for i in 0..self.columns {
            print!("   {}  ", i + 1);
        }

        print!("\n");
        loop {
            print!("{}", rows + 1); // Display row number
            let mut column = 0;
            for i in &self.spaces[rows] {
                column += 1;
                print!("  ");
                match i {
                    Some(Symbol::Cross) => bunt::print!("{$red+bold}X{/$}"),
                    Some(Symbol::Circle) => bunt::print!("{$green+bold}O{/$}"),
                    None => print!(" "),
                };

                if column < self.spaces[rows].len() {
                    bunt::print!("{$bold}  |{/$}");
                }
            }
            print!("\n");

            rows += 1;
            if rows == self.rows {
                break;
            }

            for i in 1..self.columns {
                if i == 1 {
                    print!(" "); // Space to account for the spacing due to number
                }
                bunt::print!("{$bold}-----+{/$}");
            }
            bunt::print!("{$bold}-----\n{/$}");
        }
        print!("\n");
    }
}

fn main() {
    let game = Game::new();
    Game::run(game);
}
