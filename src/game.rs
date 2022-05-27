use std::fmt::{self, Display};

const BOARD_SIZE: usize = 3;

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

#[derive(Debug)]
pub enum Error {
    BoardFull,
    GameDone,    
    PlaceFilled {x: usize, y: usize},
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BoardFull => write!(f, "Board is full"),
            Error::GameDone => write!(f, "Game is already done"),
            Error::PlaceFilled {x, y} => write!(f, "The place {}, {} is filled", x, y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Winner {
    O,
    X,
    Tie,
}

impl Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Winner::O => write!(f, "O is the winner !"),
            Winner::X => write!(f, "X won this game !"),
            Winner::Tie => write!(f, "no winner !"),
        }
    }
}

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

    pub fn play(&mut self, row: usize, col: usize) -> Result<(), Error> {
        // Take care of the three errors.
        if self.board[row][col].is_some() {
            return Err(Error::PlaceFilled {x: row, y: col})
        } else if self.board.iter().all(|x| x.iter().all(|x| x.is_some())) {
            return Err(Error::BoardFull)
        } else if self.winner.is_some() {
            return Err(Error::GameDone)
        } else {
            // If there is no error, let the player place his or her tile.
            self.board[row][col] = Some(self.current_piece);

            // Look if there is a winner with the last move.
            self.winner = self.check_winner(row, col);

            // Change the current piece.
            self.current_piece = self.current_piece.switch();

            // Return () so we know everything went well.
            return Ok(())
        }
    }

    fn check_winner(&self, row: usize, col: usize) -> Option<Winner> {
        // Three ways to win. Row, Col, Diag. Only look for the last
        // played tile. When looking if all piece are equal, change the 
        // the None to the opossing player so the comparison can happen.
        let current_row = self.board[row].iter().all(|x| x.unwrap_or(self.current_piece.switch()) == self.current_piece);
        let current_col = self.board
            .iter()
            .map(|x| x[col])
            .all(|x| x.unwrap_or(self.current_piece.switch()) == self.current_piece);
        
        // Diagonal 0 is when both row and col are equal.
        // |x|o|o|
        // |o|x|o|
        // |o|o|x|
        let diag0 = if row == col {
            (0..self.board.len())
                .all(|x| self.board[x][x].unwrap_or(self.current_piece.switch()) == self.current_piece)
        } else {
            false
        };

        // Diagonal 1 is the opposing diagonal.
        // |o|o|x|
        // |o|x|o|
        // |x|o|o|
        let diag1 = if row + col == self.board.len() - 1 {
            (0..self.board.len())
                .zip((0..self.board.len()).rev())
                .all(|(x, y)| self.board[x][y].unwrap_or(self.current_piece.switch()) == self.current_piece) 
        } else {
            false
        };
       
        // Return a winner if there is one.
        if current_row || current_col || diag0 || diag1 {
            match self.current_piece {
                Tile::X => return Some(Winner::X),
                Tile::O => return Some(Winner::O),
            }
        } else if self.board
                .iter()
                .all(|x| x.iter().all(|b| b.is_some())) {
            return Some(Winner::Tie)
        } else  {
            return None
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_game() {
        let game = Game::new();
        assert_eq!(game.board, [[None; 3]; 3]);
    }

    #[test]
    fn play_game() {
        let mut game = Game::new();
        game.play(0, 0);
        assert_eq!(game.board[0][0], Some(Tile::X));
        assert_eq!(game.board[0][1], None);
    }

    #[test]
    fn play_game_till_row_winner() {
        let mut game = Game::new();
        // X playing.
        game.play(0, 0).unwrap_or_else(|err|
            panic!()
        );
        // O playing.
        game.play(1, 1).unwrap_or_else(|err|
            panic!()
        );
        // X playing.
        game.play(0, 1).unwrap_or_else(|err|
            panic!()
        );
        // O playing.
        game.play(1, 2).unwrap_or_else(|err|
            panic!()
        );
        // X playing.
        game.play(0, 2).unwrap_or_else(|err|
            panic!()
        );
        assert_eq!(game.winner, Some(Winner::X));
    }

    #[test]
    fn play_game_till_col_winner() {
        let mut game = Game::new();
        // X playing.
        game.play(0, 0).unwrap_or_else(|err|
            panic!()
        );
        // O playing.
        game.play(1, 1).unwrap_or_else(|err|
            panic!()
        );
        // X playing.
        game.play(1, 0).unwrap_or_else(|err|
            panic!()
        );
        // O playing.
        game.play(2, 2).unwrap_or_else(|err|
            panic!()
        );
        // X playing.
        game.play(2, 0).unwrap_or_else(|err|
            panic!()
        );
        assert_eq!(game.winner, Some(Winner::X));
    }

    #[test]
    fn play_game_till_diag0_winner() {
        let mut game = Game::new();
        // X playing.
        game.play(0, 0).unwrap_or_else(|err|
            panic!()
        );
        // O playing.
        game.play(0, 1).unwrap_or_else(|err|
            panic!()
        );
        // X playing.
        game.play(1, 1).unwrap_or_else(|err|
            panic!()
        );
        // O playing.
        game.play(2, 0).unwrap_or_else(|err|
            panic!()
        );
        // X playing.
        game.play(2, 2).unwrap_or_else(|err|
            panic!()
        );
        assert_eq!(game.winner, Some(Winner::X));
    }

    #[test]
    fn play_game_till_diag1_winner() {
        let mut game = Game::new();
        // X playing.
        game.play(0, 2).unwrap_or_else(|err|
            panic!()
        );
        // O playing.
        game.play(0, 1).unwrap_or_else(|err|
            panic!()
        );
        // X playing.
        game.play(1, 1).unwrap_or_else(|err|
            panic!()
        );
        // O playing.
        game.play(2, 1).unwrap_or_else(|err|
            panic!()
        );
        // X playing.
        let err = game.play(2, 0);
        println!("{:?}", err);
        println!("{:?}", game.board);
        assert_eq!(game.winner, Some(Winner::X));
    }

    #[test]
    #[should_panic]
    fn place_already_taken() {
        let mut game = Game::new();
        // X playing.
        game.play(0, 2).unwrap_or_else(|err|
            panic!()
        );
        // O playing.
        game.play(0, 2).unwrap_or_else(|err|
            panic!()
        );
    }

}
