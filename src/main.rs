use words::{Bag, Board, Rack};
use std::io;

fn main() -> io::Result<()>{

    
    let _ = run();

    // let mut board = Board::new();

    // board.put_tile('h', 7, 7);
    // board.put_tile('e', 7, 8);
    // board.put_tile('l', 7, 9);
    // board.put_tile('l', 7, 10);
    // board.put_tile('o', 7, 11);

    // board.submit();
    
    // board.put_tile('F', 5, 11);
    // board.put_tile('O', 6, 11);
    // board.put_tile('D', 9, 11);
    
    // board.submit();

    // board.show();


    Ok(())
}

fn run() -> io::Result<()> {
    use std::io::{stdin,stdout,Write};


    let mut board = Board::new("dict.txt".to_string());
    let mut rack = Rack::new();
    let mut bag = Bag::new();
    rack.draw(&mut bag);

    let mut score = 0;
    
    println!("New scrabble game!");
    board.show();
    rack.show();
    println!("There are {} tiles in the bag.", bag.size());
    println!("Score: {score}");
    loop {
        print!("> ");
        let mut s=String::new();
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        let mut iter = s.split_whitespace();
        let cmd = iter.next().expect("First iteration should always exist.");

        if cmd == "show" {
            board.show();
            rack.show();
            println!("There are {} tiles in the bag.", bag.size());
            println!("Score: {score}");
        }
        else if cmd == "put" {
            let tile = get_char(&mut iter);
            if tile.is_none() {
                println!("Invalid command. Please try again.");
                continue;
            }
            let tile = tile.unwrap();
            let row = get_usize(&mut iter);
            if row.is_none() {
                println!("Invalid command. Please try again.");
                continue;
            }
            let row = row.unwrap();
            let col = get_usize(&mut iter);
            if col.is_none() {
                println!("Invalid command. Please try again.");
                continue;
            }
            let col = col.unwrap();

            board.put_tile_from_rack(&mut rack, tile, row, col);
        }
        else if cmd == "wa" {
            let word = get_arg(&mut iter);
            if word.is_none() {
                println!("Invalid command. Please try again.");
                continue;
            }
            let word = word.unwrap();
            let row = get_usize(&mut iter);
            if row.is_none() {
                println!("Invalid command. Please try again.");
                continue;
            }
            let row = row.unwrap();
            let col = get_usize(&mut iter);
            if col.is_none() {
                println!("Invalid command. Please try again.");
                continue;
            }
            let col = col.unwrap();

            board.write_across_from_rack(&mut rack, word, row, col);
        }
        else if cmd == "wd" {
            let word = get_arg(&mut iter);
            if word.is_none() {
                println!("Invalid command. Please try again.");
                continue;
            }
            let word = word.unwrap();
            let row = get_usize(&mut iter);
            if row.is_none() {
                println!("Invalid command. Please try again.");
                continue;
            }
            let row = row.unwrap();
            let col = get_usize(&mut iter);
            if col.is_none() {
                println!("Invalid command. Please try again.");
                continue;
            }
            let col = col.unwrap();

            board.write_down_from_rack(&mut rack, word, row, col);
        }
        else if cmd == "swap" {
            let mut to_swap = Vec::new();
            while let Some(c) = get_char(&mut iter) {
                to_swap.push(c.to_ascii_uppercase());
            }
            rack.swap(&mut bag, to_swap);
        }
        else if cmd == "exit" {
            return Ok(());
        }
        else if cmd == "submit" {
            let score_delta = board.submit();
            
            if score_delta == 0 {
                println!("Submission not accepted. Try again.");
                board.unstage_to_rack(&mut rack);
            }
            else {
                score += score_delta;
                rack.draw(&mut bag);

                board.show();
                rack.show();
                println!("There are {} tiles in the bag.", bag.size());
                println!("Score: {score}");
            }
        }
        else if cmd == "unstage" {
            board.unstage_to_rack(&mut rack);
            board.show();
            rack.show();
        }
        else {
            println!("Invalid command. Try again.");
        }
    }
}

fn get_arg<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<String> {
    match iter.next() {
        Some(s) => Some(s.to_string()),
        None => None
    }
}

fn get_char<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<char> {
    let s = get_arg(iter);
    match s {
        Some(t) => {
            if t.len() != 1 { None } else { Some(t.chars().nth(0).unwrap()) } 
        }
        None => None,
    }
}

fn get_usize<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Option<usize> {
    let s = get_arg(iter);
    match s {
        Some(t) => t.parse().ok(),
        None => None,
    }
}
