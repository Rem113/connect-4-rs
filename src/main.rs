use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;

use colorize::AnsiColor;

const N_ROWS: usize = 7;
const N_COLUMNS: usize = 9;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
enum Cell {
    #[default]
    Empty,
    Red,
    Blue,
}

#[derive(Debug, Clone, Copy)]
enum Player {
    Red,
    Blue,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let representation = match self {
            Cell::Empty => "●".black(),
            Cell::Blue => "●".blue(),
            Cell::Red => "●".red(),
        };

        write!(f, "{}", representation)
    }
}

#[derive(Debug, Clone)]
struct Column {
    cells: [Cell; N_ROWS],
    index: usize,
}

impl Default for Column {
    fn default() -> Self {
        Self {
            cells: [Cell::default(); N_ROWS],
            index: 0,
        }
    }
}

impl Column {
    fn drop_piece(&mut self, piece: Player) {
        let new_cell_state = match piece {
            Player::Red => Cell::Red,
            Player::Blue => Cell::Blue,
        };

        self.cells[self.index] = new_cell_state;
        self.index += 1;
    }

    fn is_full(&self) -> bool {
        self.index == N_ROWS
    }
}

#[derive(Debug, Default)]
struct Board {
    columns: [Column; N_COLUMNS],
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "-".repeat(N_COLUMNS * 4 + 1))?;

        for j in 0..N_COLUMNS {
            write!(f, "| {} ", j + 1)?;
        }

        writeln!(f, "|")?;

        writeln!(f, "{}", "-".repeat(N_COLUMNS * 4 + 1))?;

        for i in (0..N_ROWS).rev() {
            for j in 0..N_COLUMNS {
                write!(f, "| {} ", self.columns[j].cells[i])?;
            }

            writeln!(f, "|")?;
        }

        writeln!(f, "{}", "-".repeat(N_COLUMNS * 4 + 1))
    }
}

enum GameResult {
    RedWon,
    BlueWon,
    Tie,
}

struct Game {
    next_player: Player,
    board: Board,
}

impl Game {
    fn new() -> Self {
        Self {
            next_player: Player::Blue,
            board: Board::default(),
        }
    }

    fn play_turn(&mut self) {
        let column = loop {
            println!("{}", self.board);
            print!(
                "{}'s turn, enter a column index: ",
                match self.next_player {
                    Player::Red => "Red".red(),
                    Player::Blue => "Blue".blue(),
                }
            );
            io::stdout().flush().ok();

            match read_positive_usize() {
                Ok(column) if column <= N_COLUMNS && self.board.columns[column - 1].is_full() => {
                    println!("Column {} is full, please select another one.", column)
                }
                Ok(column) if column <= N_COLUMNS => break column,
                Ok(_) => println!("Please enter a number smaller than {}.", N_COLUMNS),
                Err(_) => println!("Please input a number between 1 and {}.", N_COLUMNS),
            };
        };

        self.board.columns[column - 1].drop_piece(self.next_player);
        self.next_player = match self.next_player {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        };
    }

    fn play(&mut self) -> GameResult {
        loop {
            self.play_turn();

            // if let Some(result) = self.check_for_result() {
            //     return result;
            // }
        }
    }

    fn check_for_result(&self) -> Option<GameResult> {
        fn check_for_horizontal_victory() -> Option<GameResult> {
            unimplemented!()
        }

        fn check_for_vertical_victory() -> Option<GameResult> {
            unimplemented!()
        }

        fn check_for_diagonal_victory() -> Option<GameResult> {
            unimplemented!()
        }

        check_for_horizontal_victory()
            .or(check_for_vertical_victory())
            .or(check_for_diagonal_victory())
    }
}

fn read_positive_usize() -> io::Result<usize> {
    let mut column = String::new();
    io::stdin().read_line(&mut column)?;
    let column = column
        .trim()
        .parse()
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))?;

    if column > N_COLUMNS || column == 0 {
        return Err(io::Error::from(io::ErrorKind::InvalidInput));
    }

    Ok(column)
}

fn main() {
    let mut game = Game::new();
    game.play();
}
