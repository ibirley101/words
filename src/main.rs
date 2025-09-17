use clap::Parser;
use std::io;
use words::game::{Bag, Board};
use words::player::Player;

#[derive(Debug, Clone, clap::ValueEnum)]
enum PlayerType {
    Human,
    HumanNoRack,
    CPU,
    None,
}

// Program to assist in Scrabble-like games.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // number of human players
    #[arg(value_enum, default_value_t = PlayerType::Human)]
    player1: PlayerType,

    #[arg(value_enum, default_value_t = PlayerType::CPU)]
    player2: PlayerType,

    #[arg(value_enum, default_value_t = PlayerType::None)]
    player3: PlayerType,

    #[arg(value_enum, default_value_t = PlayerType::None)]
    player4: PlayerType,

    // whether human players have to play from a rack
    #[arg(short, long, default_value_t = false)]
    rackless: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let players = initialize_players(args);

    if players.is_empty() {
        println!("No players initialized. Please see --help.");
        return Ok(());
    }

    let _ = run(players);

    Ok(())
}

fn initialize_players(args: Args) -> Vec<Box<Player>> {
    let player_types = vec![args.player1, args.player2, args.player3, args.player4];
    let mut players = Vec::new();

    let mut id = 1;

    for player_type in player_types {
        let new_player = match player_type {
            PlayerType::Human => Some(Player::new(id, false, false)),
            PlayerType::HumanNoRack => Some(Player::new(id, true, false)),
            PlayerType::CPU => Some(Player::new(id, false, true)),
            PlayerType::None => None,
        };
        
        if let Some(p) = new_player {
            players.push(Box::new(p))
        }

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
