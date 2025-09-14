use words::game::{Bag, Board, Rack};
use words::player::Player;
use std::io;
use clap::Parser;

// Program to assist in Scrabble-like games.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // number of human players
    #[arg(long)]
    num_humans: i32,

    // number of computer players
    #[arg(long)]
    num_cpus: i32,

    // whether human players have to play from a rack
    #[arg(short, long, default_value_t=false)]
    rackless: bool
}

fn main() -> io::Result<()>{
    let args = Args::parse();
    
    let players = initialize_players(args);

    let _ = run(players);

    Ok(())
}

fn initialize_players(args: Args) -> Vec<Box<Player>> {
    let mut players = Vec::new();

    let mut id = 1;

    for _ in 0..args.num_humans{
        let new_player = Player::new(id, args.rackless, false);
        players.push(Box::new(new_player));
        id += 1;
    }

    for _ in 0..args.num_cpus{
        let new_player = Player::new(id, false, true);
        players.push(Box::new(new_player));
        id += 1;
    }

    players
}

fn run(mut players: Vec<Box<Player>>) -> io::Result<()> {
    let mut board = Board::new("dict.txt".to_string(), "partials_dict.txt".to_string());
    let mut bag = Bag::new();


    while !bag.is_empty() {
        for player in &mut players {
            player.play_turn(&mut board, &mut bag);
        }

        board.show();

    }

    Ok(())
}

