use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::time::Instant;
use words::Bag;
use words::Board;

fn main() -> io::Result<()>{

    
    run();

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

fn read_word_list<P>(filename: P) -> io::Result<Vec<String>> where P: AsRef<Path>{
    let file: File = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut word_list = Vec::new();

    for line in lines {
        word_list.push(line?);
    }

    Ok(word_list)
}

fn run() -> io::Result<()> {
    use std::io::{stdin,stdout,Write};


    let mut board = Board::new("dict.txt".to_string());

    println!("New scrabble game!");
    board.show();
    loop {
        let mut s=String::new();
        print!("> ");

        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        if s == "show" {
            board.show();
        }
        else if s.starts_with("put") {
            let mut iter = s.split_whitespace();
            let cmd = iter.next().expect("first iteration should exist");
            if cmd != "put" {
                println!("Invalid command. Please try again.");
                continue;
            }

            let tile = iter.next().unwrap();
            let row: usize = iter.next().unwrap().parse().unwrap();
            let col: usize = iter.next().unwrap().parse().unwrap();

            board.put_tile(tile.chars().nth(0).unwrap(), row, col);
        }
        else if s.starts_with("wa") {
            let mut iter = s.split_whitespace();
            let cmd = iter.next().expect("first iteration should exist");
            if cmd != "wa" {
                println!("Invalid command. Please try again.");
                continue;
            }

            let word = iter.next().unwrap();
            let row: usize = iter.next().unwrap().parse().unwrap();
            let col: usize = iter.next().unwrap().parse().unwrap();

            board.write_across(word.to_string(), row, col);
        }
        else if s.starts_with("wd") {
            let mut iter = s.split_whitespace();
            let cmd = iter.next().expect("first iteration should exist");
            if cmd != "wd" {
                println!("Invalid command. Please try again.");
                continue;
            }

            let word = iter.next().unwrap();
            let row: usize = iter.next().unwrap().parse().unwrap();
            let col: usize = iter.next().unwrap().parse().unwrap();

            board.write_down(word.to_string(), row, col);
        }
        else if s == "exit" {
            return Ok(());
        }
        else if s == "submit" {
            board.submit();
        }
        else if s == "unstage" {
            board.unstage();
        }
        else {
            println!("Invalid command. Try again.");
        }
    }
}
