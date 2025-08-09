use std::collections::VecDeque;

use crate::game::{Bag, Board, Rack};

pub fn play_greediest_word(board: &mut Board, rack: &mut Rack) {
    // to play the greediest word, we will need to search all of the available
    // neighbors on the board. 

    // for each neighbor, we pick a tile, then verify that the tile is some substring
    // in a dictionary word. (I'm scared because that sounds like a tricky search).

    // Once we've verified that there is at least one potential dictionary word.

    let mut best_score = 0;
    let mut best_play = (String::new(), 15, 15, false); // (word, row, col, across)

    for neighbor in board.get_neighbors() {
        find_words(board, rack, neighbor.0, neighbor.1);
    }
}


fn find_words(board: &mut Board, rack: &mut Rack, row: usize, col: usize) {
    // Step 1: place a letter
    let mut tiles: VecDeque<char> = rack.get_tiles_vecdeque();
    let mut used_tiles: Vec<char> = Vec::new();

    
    
    let letter = tiles.pop_back().unwrap();
    used_tiles.push(letter);
    // Step 2: try filling across
    while !used_tiles.is_empty() {
        board.put_tile(letter, row, col);
        let substring = board.get_word_across(row, col).unwrap();
        if !board.is_word_down(row, col) {
            tiles.push_front(used_tiles.pop().unwrap());
        } else if board.substr_promising(&substring) {
            println!("{substring} is promising");
        }
    }

    // Step 3: try filling down
}