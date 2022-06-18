use std::fmt::{self, Display};

const BOARD_SIZE: usize = 3;
const LETTERS: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
const CAPLETTERS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
const RADIX: u32 = 10;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    O,
    X,
}

impl Tile {
    fn switch(self) -> Tile {
        match self {
            Tile::O => Tile::X,
            Tile::X => Tile::O,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::O => write!(f, "O"),
            Tile::X => write!(f, "X"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    row: usize,
    col: usize,
}

impl TryFrom<(usize, usize)> for Move {
    type Error = MoveError;
    fn try_from(source: (usize, usize)) -> Result<Self, MoveError> {
        let (row, col) = source;
        if row > BOARD_SIZE - 1 || col > BOARD_SIZE - 1 {
            Err(MoveError::OutOfBounds)
        } else {
            Ok(Move {row, col})
        }
    }
}

impl std::str::FromStr for Move {
    type Err = MoveError;

    fn from_str(source: &str) -> Result<Self, MoveError> {
        // Look the size of the str. If the size is more than
        // 2 it means there is an error.
        if source.chars().count() != 2 {
            return Err(MoveError::BadInput)
        } 

        let board_letters: Vec<char> = LETTERS[0..BOARD_SIZE].to_vec();
        let board_numbers: Vec<char> = (0..BOARD_SIZE).map(|i| char::from_digit(i as u32, RADIX).unwrap()).collect();

        // Collect the two chars in a Tuple. 
        let temp: Vec<char> = source.to_ascii_lowercase().chars().collect();
        let (f, s) = (temp[0], temp[1]);

        // If the first letter is a letter and inbounds.
        if board_letters.contains(&f) & board_numbers.contains(&s) {
            // Find the index where f is in board_letters.
            // Find the index where s is in board_numbers.
            Ok(Move::try_from(
                (board_letters.iter().position(|letter| letter == &f).unwrap(),
                 board_numbers.iter().position(|number| number == &s).unwrap())))?
        } else if board_numbers.contains(&f) & board_letters.contains(&s) {
            // Find the index where s is in board_letters.
            // Find the index where f is in board_numbers.
            Ok(Move::try_from(
                (board_letters.iter().position(|letter| letter == &s).unwrap(),
                board_numbers.iter().position(|number| number == &f).unwrap())))?
        } else {
            // If it isn't part of the board it is a bad input. (Could be more precise)
            Err(MoveError::BadInput)
        }
    }
}

#[derive(Debug)]
pub enum MoveError {
    OutOfBounds,
    BadInput,
}

impl Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::OutOfBounds => write!(f, "This move is out of bounds"),
            MoveError::BadInput => write!(f, "This is a bad input"),
        }
    }
}

impl std::error::Error for MoveError {}

pub struct Game {
    pub winner: Option<Winner>,
    pub board: [[Option<Tile>; BOARD_SIZE]; BOARD_SIZE],
    pub current_piece: Tile,
}

impl Game {
    pub fn new() -> Self {
        Game {
            winner: None,
            board: Default::default(),
            current_piece: Tile::X,
        }
    }

    fn is_full(&self) -> bool {
        self.board.iter().all(|x| x.iter().all(|x| x.is_some()))
    }

    pub fn play(&mut self, _move: Move) -> Result<(), GameError> {
        // Collect the row and column from the Move struc.
        let (row, col) = (_move.row, _move.col);

        // Take care of the three errors.
        if self.board[row][col].is_some() {
            return Err(GameError::PlaceFilled { x: row, y: col });
        } else if self.is_full() {
            return Err(GameError::BoardFull);
        } else if self.winner.is_some() {
            return Err(GameError::GameDone);
        } else {
            // If there is no error, let the player place his or her tile.
            self.board[row][col] = Some(self.current_piece);

            // Look if there is a winner with the last move.
            self.winner = self.check_winner(row, col);

            // Change the current piece.
            self.current_piece = self.current_piece.switch();

            // Return () so we know everything went well.
            return Ok(());
        }
    }

    fn check_horizontal(&self, row: usize) -> Option<Winner> {
        // X => Option<Tile> , current_piece = Some(Tile)
        self.board[row]
            .iter()
            .all(|tile| *tile == Some(self.current_piece))
            .then(|| self.current_piece)
            .map(Winner::from)
    }

    fn check_vetical(&self, col: usize) -> Option<Winner> {
        // X => Option<Tile> , current_piece = Some(Tile)
        self.board
            .iter()
            .map(|x| x[col])
            .all(|x| x == Some(self.current_piece))
            .then(|| self.current_piece)
            .map(Winner::from)
    }

    fn check_diagonals(&self, row: usize, col: usize) -> Option<Winner> {
        // Find better way to find diagonals
        if row == col {
            (0..self.board.len())
                .all(|x| self.board[x][x] == Some(self.current_piece))
                .then(|| self.current_piece)
                .map(Winner::from)
        } else if row + col == self.board.len() - 1 {
            (0..self.board.len()).map(|row| (row, self.board.len() - 1 - row))
                .all(|(x, y)| self.board[x][y] == Some(self.current_piece))
                .then(|| self.current_piece)
                .map(Winner::from)
        } else {
            None
        }
    }

    fn check_tie(&self) -> Option<Winner> {
        match self.is_full() {
            true => Some(Winner::Tie),
            false => None,
        }
    }

    fn check_winner(&self, row: usize, col: usize) -> Option<Winner> {
        // Return a winner if there is one.
        self.check_horizontal(row)
            .or_else(|| self.check_vetical(col))
            .or_else(|| self.check_diagonals(row, col))
            .or_else(|| self.check_tie())
    }
}


#[derive(Debug)]
pub enum GameError {
    BoardFull,
    GameDone,
    PlaceFilled { x: usize, y: usize },
}

impl Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::BoardFull => write!(f, "Board is full"),
            GameError::GameDone => write!(f, "Game is already done"),
            GameError::PlaceFilled { x, y } => write!(f, "The place {}, {} is filled", x, y),
        }
    }
}

impl std::error::Error for GameError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Winner {
    Player(Tile),
    Tie,
}

impl From<Tile> for Winner {
    fn from(source: Tile) -> Winner {
        Winner::Player(source)
    }
}

impl Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Winner::Player(Tile::O) => write!(f, "O is the winner !"),
            Winner::Player(Tile::X) => write!(f, "X won this game !"),
            Winner::Tie => write!(f, "no winner !"),
        }
    }
}

pub fn print_board(game: &Game) {
    println!(
        "\n   {}",
        (0..game.board.len())
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("--")
    );
    for (i, row) in game.board.iter().enumerate() {
        // The capital letters identifies the rows.
        let mut to_display = String::from(CAPLETTERS[i]);

        // Add a space to the char for better display.
        to_display += " ";

        // Fill the board with the Tiles already there, when None "~".
        for cell in row.iter() {
            let to_push = match cell {
                Some(Tile::O) => " O ",
                Some(Tile::X) => " X ",
                None => " ~ ",
            };
            to_display += to_push;
        }
        println!("{}", to_display);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn create_game() {
        let game = Game::new();
        assert_eq!(game.board, [[None; 3]; 3]);
    }

    #[test]
    fn create_move() {
        let _move = Move::from_str("1b").unwrap();
        assert_eq!(_move, Move::try_from((1,1)).unwrap());
    }

    #[test]
    fn create_other_move() {
        let _move = Move::from_str("0c").unwrap();
        assert_eq!(_move, Move::try_from((2, 0)).unwrap());
    }

    #[test]
    fn play_move(){
        let mut game = Game::new();
        let _move = Move::from_str("0c").unwrap();
        game.play(_move).unwrap();
        assert_eq!(game.board[2][0], Some(Tile::X));
    }

    #[test]
    fn play_game(){
        let mut game = Game::new();
        game.play(Move::from_str("0c").unwrap()).unwrap();
        game.play(Move::from_str("0b").unwrap()).unwrap();
        game.play(Move::from_str("1b").unwrap()).unwrap();
        game.play(Move::from_str("0A").unwrap()).unwrap();
        game.play(Move::from_str("2A").unwrap()).unwrap();
        assert_eq!(game.winner, Some(Winner::Player(Tile::X)));
    }
}
