use crate::{
    game::{Bag, Board, Rack},
    shell::{Shell, ShellStatus},
};
use std::io::{Write, stdin, stdout};

pub enum TurnResult {
    Score(i32),
    Swap,
    Exit,
}

pub struct Player {
    pub rack: Rack,
    pub id: i32,
    pub score: i32,
    pub rackless: bool,
    pub cpu: bool,
}

impl Player {
    pub fn new(id: i32, rackless: bool, cpu: bool) -> Self {
        if rackless && cpu {
            panic!("CPU cannot be rackless.");
        }

        Player {
            rack: Rack::new(),
            id,
            score: 0,
            rackless,
            cpu,
        }
    }

    pub fn play_turn(&mut self, board: &mut Board, bag: &mut Bag) -> TurnResult {
        println!("Player {}'s turn.", self.id);
        if !self.rackless {
            self.rack.draw(bag);
        }

        let score_delta;
        if self.cpu {
            score_delta = match self.play_turn_cpu(board, bag) {
                TurnResult::Score(n) => n,
                TurnResult::Swap => 0, // swap
                TurnResult::Exit => return TurnResult::Exit,
            }
        } else {
            score_delta = match self.play_turn_player(board, bag) {
                TurnResult::Score(n) => n,
                TurnResult::Swap => 0, // swap
                TurnResult::Exit => return TurnResult::Exit,
            }
        }
        self.score += score_delta;
        println!("Score: {}\n", self.score);
        TurnResult::Score(score_delta)
    }

    fn play_turn_cpu(&mut self, board: &mut Board, _bag: &mut Bag) -> TurnResult {
        let (word, _, row, col, across) = find_greediest_word(board, &mut self.rack);
        if across {
            board.write_across_from_rack(&mut self.rack, word, row, col);
        } else {
            board.write_down_from_rack(&mut self.rack, word, row, col);
        }
        let score = board.submit();

        TurnResult::Score(score)
    }

    fn play_turn_player(&mut self, board: &mut Board, bag: &mut Bag) -> TurnResult {
        let mut shell = Shell::new(bag, board, self);
        match shell.main_loop() {
            ShellStatus::Exit => TurnResult::Exit,
            ShellStatus::Submit(score) => TurnResult::Score(score),
            _ => TurnResult::Exit, // should not be possible
        }
    }
}

pub fn find_greediest_word(board: &mut Board, rack: &Rack) -> (String, i32, usize, usize, bool) {
    // to play the greediest word, we will need to search all of the available
    // neighbors on the board.

    // for each neighbor, we pick a tile, then verify that the tile is some substring
    // in a dictionary word. (I'm scared because that sounds like a tricky search).

    // Once we've verified that there is at least one potential dictionary word.
    let mut best_play: (String, i32, usize, usize, bool) = (String::new(), 0, 15, 15, false);
    for neighbor in board.get_neighbors() {
        let best_opt = find_words(board, rack, neighbor.0, neighbor.1, best_play.1);
        if best_opt.1 > best_play.1 {
            best_play = best_opt;
        }
    }

    best_play
}

fn find_words_across(
    board: &mut Board,
    tiles: &Vec<char>,
    row: usize,
    col: usize,
    best: &mut (String, i32, usize, usize),
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
        if score > best.1 {
            best.0 = substr.clone();
            best.1 = score;
            best.2 = row;
            best.3 = board.get_leftmost_col(row, col).unwrap();
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
    best: &mut (String, i32, usize, usize),
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
        if score > best.1 {
            best.0 = substr.clone();
            best.1 = score;
            best.2 = board.get_upmost_row(row, col).unwrap();
            best.3 = col;
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
) -> (String, i32, usize, usize, bool) {
    let mut best_across = (String::new(), best_score, 15, 15);
    find_words_across(board, &rack.get_tiles_vec(), row, col, &mut best_across);

    let mut best_down = (String::new(), best_score, 15, 15);
    find_words_down(board, &rack.get_tiles_vec(), row, col, &mut best_down);

    if best_across.1 > best_down.1 {
        (
            best_across.0,
            best_across.1,
            best_across.2,
            best_across.3,
            true,
        )
    } else {
        (best_down.0, best_down.1, best_down.2, best_down.3, false)
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
