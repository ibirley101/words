use crate::game::{Board, Rack};

pub struct WordChoice {
    pub word: String,
    pub score: i32,
    pub row: usize,
    pub col: usize,
    pub across: bool,
}

pub fn find_greediest_word(board: &mut Board, rack: &Rack) -> WordChoice {
    // to play the greediest word, we will need to search all of the available
    // neighbors on the board.

    // for each neighbor, we pick a tile, then verify that the tile is some substring
    // in a dictionary word. (I'm scared because that sounds like a tricky search).

    // Once we've verified that there is at least one potential dictionary word.
    let mut best_play= WordChoice {word: String::new(), score: 0, row: 15, col: 15, across: false};
    for neighbor in board.get_neighbors() {
        let best_for_here = find_words(board, rack, neighbor.0, neighbor.1, best_play.score);
        if best_for_here.score > best_play.score {
            best_play = best_for_here;
        }
    }

    best_play
}

fn find_words_across(
    board: &mut Board,
    tiles: &Vec<char>,
    row: usize,
    col: usize,
    best: &mut WordChoice,
) {
    let across_candidates;
    if board.get_tile(row, col) == '-' {
        across_candidates = vec![(row, col)];
    } else {
        across_candidates = get_across_candidates(board, row, col);
    }

    let substr = board.get_word_across(row, col).unwrap_or(String::new());
    if !board.is_word_down(row, col) || !board.substr_promising(&substr) {
        return;
    }

    if board.is_word_across(row, col) {
        let score = board.score();
        if score > best.score {
            best.word = substr.clone();
            best.score = score;
            best.row = row;
            best.col = board.get_leftmost_col(row, col).unwrap();
            best.across = true;
        }
    }

    if tiles.is_empty() {
        return;
    }

    for candidate in across_candidates {
        for (i, letter) in tiles.iter().enumerate() {
            board.put_tile(*letter, candidate.0, candidate.1);
            let mut tiles_copy = tiles.clone();
            tiles_copy.remove(i);
            find_words_across(board, &tiles_copy, candidate.0, candidate.1, best);
            board.remove_tile(candidate.0, candidate.1);
        }
    }
}

fn find_words_down(
    board: &mut Board,
    tiles: &Vec<char>,
    row: usize,
    col: usize,
    best: &mut WordChoice,
) {
    let down_candidates;
    if board.get_tile(row, col) == '-' {
        down_candidates = vec![(row, col)];
    } else {
        down_candidates = get_down_candidates(board, row, col);
    }

    let substr = board.get_word_down(row, col).unwrap_or(String::new());
    if !board.is_word_across(row, col) || !board.substr_promising(&substr) {
        return;
    }

    if board.is_word_down(row, col) {
        let score = board.score();
        if score > best.score {
            best.word = substr.clone();
            best.score = score;
            best.row = board.get_upmost_row(row, col).unwrap();
            best.col = col;
            best.across = false;
        }
    }

    if tiles.is_empty() {
        return;
    }

    for candidate in down_candidates {
        for (i, letter) in tiles.iter().enumerate() {
            board.put_tile(*letter, candidate.0, candidate.1);
            let mut tiles_copy = tiles.clone();
            tiles_copy.remove(i);
            find_words_down(board, &tiles_copy, candidate.0, candidate.1, best);
            board.remove_tile(candidate.0, candidate.1);
        }
    }
}

fn find_words(
    board: &mut Board,
    rack: &Rack,
    row: usize,
    col: usize,
    best_score: i32,
) -> WordChoice {
    let mut best_across = WordChoice {word: String::new(), score: best_score, row: 15, col: 15, across: true};
    find_words_across(board, &rack.get_tiles_vec(), row, col, &mut best_across);

    let mut best_down = WordChoice {word: String::new(), score: best_score, row: 15, col: 15, across: false};
    find_words_down(board, &rack.get_tiles_vec(), row, col, &mut best_down);

    if best_across.score > best_down.score {
        best_across
    } else {
        best_down
    }
}

fn get_across_candidates(board: &Board, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    let leftmost = board.get_leftmost_col(row, col).unwrap();
    let rightmost = board.get_rightmost_col(row, col).unwrap();

    result.append(&mut get_across_neighbors(board, row, leftmost));

    if leftmost != rightmost {
        result.append(&mut get_across_neighbors(board, row, rightmost));
    }

    result
}

fn get_across_neighbors(board: &Board, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    let top_edge = row == 0;
    let bottom_edge = row == 14;
    let left_edge = col == 0;
    let right_edge = col == 14;

    // There's probably a better way to do this, but I just went with this.
    if left_edge {
        // non-corner left
        if board.get_tile(row, col + 1) == '-' {
            result.push((row, col + 1));
        }
    } else if bottom_edge && right_edge {
        // bottom-right corner
        if board.get_tile(row, col - 1) == '-' {
            result.push((row, col - 1));
        }
    } else if bottom_edge {
        // non-corner bottom
        if board.get_tile(row, col - 1) == '-' {
            result.push((row, col - 1));
        }
        if board.get_tile(row, col + 1) == '-' {
            result.push((row, col + 1));
        }
    } else if right_edge {
        // non-corner right
        if board.get_tile(row, col - 1) == '-' {
            result.push((row, col - 1));
        }
    } else if top_edge {
        // non-corner top
        if board.get_tile(row, col - 1) == '-' {
            result.push((row, col - 1));
        }
        if board.get_tile(row, col + 1) == '-' {
            result.push((row, col + 1));
        }
    } else {
        // non-edge case
        if board.get_tile(row, col - 1) == '-' {
            result.push((row, col - 1));
        }
        if board.get_tile(row, col + 1) == '-' {
            result.push((row, col + 1));
        }
    }

    result
}

fn get_down_candidates(board: &Board, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    let upmost = board.get_upmost_row(row, col).unwrap();
    let downmost = board.get_downmost_row(row, col).unwrap();

    result.append(&mut get_down_neighbors(board, upmost, col));

    if upmost != downmost {
        result.append(&mut get_down_neighbors(board, downmost, col));
    }

    result
}

fn get_down_neighbors(board: &Board, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    let top_edge = row == 0;
    let bottom_edge = row == 14;

    // There's probably a better way to do this, but I just went with this.
    if top_edge {
        if board.get_tile(row + 1, col) == '-' {
            result.push((row + 1, col));
        }
    } else if bottom_edge {
        if board.get_tile(row - 1, col) == '-' {
            result.push((row - 1, col));
        }
    } else {
        if board.get_tile(row - 1, col) == '-' {
            result.push((row - 1, col));
        }
        if board.get_tile(row + 1, col) == '-' {
            result.push((row + 1, col));
        }
    }
    result
}
