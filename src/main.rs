mod game;

use game::{print_board, Game, Move};
use std::io;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            .expect("This isn't a valid input.");

        // Create the next move.
        let next_move = match Move::from_str(next_move.trim()) {
            Ok(x) => x,
            Err(x) => {print!("{}", x);
                                  continue}
        };

        // Play the move.
        match game.play(next_move){
            Ok(_) => (),
            Err(x) => print!("{}", x)
        };
    }

    // Print the board.
    print_board(&game);

    // Know this is some because the loop ended.
    println!("The game is over : {}", game.winner.unwrap());

    Ok(())

}
