mod game;

use std::io;
use game::{Game, Tile};

const RADIX: u32 = 10;

fn print_board(game: &Game) {
    
    println!("\n   {}", (0..game.board.len()).into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("--"));
    for (i, row) in game.board.iter().enumerate() {
        let mut to_display = match i {
            0 => String::from("A "),
            1 => String::from("B "),
            2 => String::from("C "),
            3 => String::from("D "),
            4 => String::from("E "),
            5 => String::from("F "),
            _ => String::from("  "),
        };
        for cell in row.iter() {
            let to_push = match cell {
                Some(Tile::O) => " O ",
                Some(Tile::X) => " X ",
                None => " ~ "
            };
            to_display += to_push;
        }
        println!("{}", to_display);
    }
}

fn main() {
    // Create a game.
    let mut game = Game::new();

    // Enter in a while loop that will run until we have a winner.
    while !game.winner.is_some() {
        // Print the board.
        print_board(&game);

        // Print the current player.
        println!("Current player : {}", game.current_piece);

        // Ask where the current player wants to play.
        println!("Where do you want to play ? (e.g. 1A, 0b)");

        let mut next_move = String::new();
        io::stdin()
            .read_line(&mut next_move)
            .expect("This isn't a valid move.");

        // Get the row and column for the next move.
        let mut next_move_vec: Vec<char> = next_move.trim().chars().collect();

        if next_move_vec.is_empty() {
            println!("Not a valid move");
            continue
        } if next_move_vec.len() != 2 {
            println!("Not a valid move");
            continue 
        }

        // Look at the first element, rev if first is letter.
        if next_move_vec[0].is_ascii_alphabetic() {
            next_move_vec.reverse();
        }

        // Change the row to a usize.
        let row: usize = match next_move_vec[1] {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            'd' | 'D' => 3,
            'e' | 'E' => 4,
            'f' | 'F' => 5,
            _ => {
                println!("Not a valid row");
                continue
            },
        };
        let col: usize = next_move_vec[0]
            .to_digit(RADIX)
            .unwrap() as usize;

        if col > game.board.len() - 1 {
            println!("Not a valid col");
            continue
        }

        // Play the move.
        match game.play(row, col) {
            Err(x) => println!("{}", x),
            Ok(_) => (),
        };
    }

    // Print the board.
    print_board(&game);

    // Know this is some because the loop ended.
    println!("The game is over : {}", game.winner.unwrap());

}
