use words::game::{Bag, Board, Rack};
use words::player::{self, Player};
use std::io;

fn main() -> io::Result<()>{

    
    let _ = run();

    Ok(())
}

fn run() -> io::Result<()> {

    let mut board = Board::new("dict.txt".to_string(), "partials_dict.txt".to_string());
    let mut bag = Bag::new();

    // initialize a two player game, one human, one cpu
    let mut rack1 = Rack::new();
    let mut rack2 = Rack::new();
    let mut player1 = Player::new(&mut rack1, 1, false, true);
    let mut player2 = Player::new(&mut rack2, 2, false, true);

    while !bag.is_empty() {
        player1.play_turn(&mut board, &mut bag);
        player2.play_turn(&mut board, &mut bag);

        board.show();
    }

    Ok(())
}

