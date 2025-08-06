use std::fmt::Write;
use rand::seq::SliceRandom;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

const LETTER_SCORES: [i32; 26] = [1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10];

pub fn score_letter(letter: char) -> i32 {
    if letter == '*' {
        return 0;
    }

    LETTER_SCORES[(letter as usize) - ('A' as usize)]
}

pub struct Bag {
    pub tiles: Vec<char>,
}

impl Bag {
    pub fn new() -> Self {
        let mut tiles = vec![
            'A','A','A','A','A','A','A','A','A',
            'B','B',
            'C','C',
            'D','D','D','D',
            'E','E','E','E','E','E','E','E','E','E','E','E',
            'F','F',
            'G','G','G',
            'H','H',
            'I','I','I','I','I','I','I','I','I',
            'J',
            'K',
            'L','L','L','L',
            'M','M',
            'N','N','N','N','N','N',
            'O','O','O','O','O','O','O','O',
            'P','P',
            'Q',
            'R','R','R','R','R','R',
            'S','S','S','S',
            'T','T','T','T','T','T',
            'U','U','U','U',
            'V','V',
            'W','W',
            'X',
            'Y','Y',
            'Z',
        ];

        let mut rng = rand::rng();
        tiles.shuffle(&mut rng);

        Bag { tiles: tiles }
    }

    pub fn draw(&mut self, rack: &mut Vec<char>, n: i32){
        for _ in 0..n {
            let tile = self.tiles.pop();
            match tile {
                Some(c) => rack.push(c),
                None => return,
            }
        }
    }
}

pub struct Board {
    board: Vec<Vec<Space>>,
    staged_spaces: Vec<(usize, usize)>,
    word_list: Vec<String>
}

impl Board {
    fn read_word_list<P>(filename: P) -> io::Result<Vec<String>> where P: AsRef<Path>{
        let file: File = File::open(filename)?;
        let lines = io::BufReader::new(file).lines();
        let mut word_list = Vec::new();

        for line in lines {
            word_list.push(line?);
        }

        Ok(word_list)
    }

    pub fn new(dict_path: String ) -> Self {
        let word_list = Board::read_word_list(dict_path).unwrap();
        
        let id = Space{tile: '-', letter_mult: 1, word_mult: 1, val: 0};
        let dl = Space{tile: '-', letter_mult: 2, word_mult: 1, val: 0};
        let tl = Space{tile: '-', letter_mult: 3, word_mult: 1, val: 0};
        let dw = Space{tile: '-', letter_mult: 1, word_mult: 2, val: 0};
        let tw = Space{tile: '-', letter_mult: 1, word_mult: 3, val: 0};

        let mut board: Vec<Vec<Space>> = Vec::new();

        board.push(vec![tw.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), tw.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), tw.clone()]);
        board.push(vec![id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), tl.clone(), id.clone(), id.clone(), id.clone(), tl.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone()]);
        board.push(vec![id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone()]);
        board.push(vec![dl.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), dl.clone()]);
        board.push(vec![id.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), id.clone()]);
        board.push(vec![id.clone(), tl.clone(), id.clone(), id.clone(), id.clone(), tl.clone(), id.clone(), id.clone(), id.clone(), tl.clone(), id.clone(), id.clone(), id.clone(), tl.clone(), id.clone()]);
        board.push(vec![id.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), id.clone()]);
        board.push(vec![tw.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), tw.clone()]);
        board.push(vec![id.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), id.clone()]);
        board.push(vec![id.clone(), tl.clone(), id.clone(), id.clone(), id.clone(), tl.clone(), id.clone(), id.clone(), id.clone(), tl.clone(), id.clone(), id.clone(), id.clone(), tl.clone(), id.clone()]);
        board.push(vec![id.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), id.clone()]);
        board.push(vec![dl.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), dl.clone()]);
        board.push(vec![id.clone(), id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone(), id.clone()]);
        board.push(vec![id.clone(), dw.clone(), id.clone(), id.clone(), id.clone(), tl.clone(), id.clone(), id.clone(), id.clone(), tl.clone(), id.clone(), id.clone(), id.clone(), dw.clone(), id.clone()]);
        board.push(vec![tw.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), id.clone(), tw.clone(), id.clone(), id.clone(), id.clone(), dl.clone(), id.clone(), id.clone(), tw.clone()]);

        Board { board: board, staged_spaces: Vec::new(), word_list: word_list }
    }

    pub fn put_tile(&mut self, tile: char, row: usize, col: usize) {
        // add logic to keep track of unsubmitted characters, kind of like git staging/committing.
        // add logic to prevent putting a tile on a filled space

        
        if self.board[row][col].tile != '-' {
            return; // not empty
        }
        
        let tile: char = tile.to_ascii_uppercase();
        self.staged_spaces.push((row, col));

        self.board[row][col].set_char(tile);
    }


    pub fn get_word_down(&self, row: usize, col: usize) -> Option<String> {
        // given a space in board, return the up/down word that it belongs to, if one exists.
        if self.board[row][col].tile == '-' {
            return None;
        }

        // get to the beginning of the word
        let mut start = row;
        while start > 0 && self.board[start-1][col].tile != '-' {
            start -= 1;
        }

        let mut end = row;
        while end < 14 && self.board[end+1][col].tile != '-' {
            end += 1;
        }

        // one letter words are not allowed
        if start == end {
            return None;
        }

        let mut word = String::new();

        for row in start..=end {
            word.push(self.board[row][col].tile);
        }

        Some(word)
    }

    pub fn get_word_across(&self, row: usize, col: usize) -> Option<String> {
        // given a space in board, return the across word that it belongs to, if one exists.
        if self.board[row][col].tile == '-' {
            return None;
        }

        // get to the beginning of the word
        let mut start = col;
        while start > 0 && self.board[row][start-1].tile != '-' {
            start -= 1;
        }

        let mut end = col;
        while end < 14 && self.board[row][end+1].tile != '-' {
            end += 1;
        }

        // one letter words are not allowed
        if start == end {
            return None;
        }

        let mut word = String::new();

        for col in  start..=end {
            word.push(self.board[row][col].tile);
        }

        Some(word)
    }

    pub fn get_leftmost_col(&self, row: usize, col: usize) -> Option<usize> {
        if self.board[row][col].tile == '-' {
            return None;
        }

        let mut start = col;
        while start > 0 && self.board[row][start-1].tile != '-' {
            start -= 1;
        }

        Some(start)
    }

    pub fn get_rightmost_col(&self, row: usize, col: usize) -> Option<usize> {
        if self.board[row][col].tile == '-' {
            return None;
        }

        let mut end = col;
        while end < 14 && self.board[row][end+1].tile != '-' {
            end += 1;
        }

        Some(end)
    }

    pub fn get_upmost_row(&self, row: usize, col: usize) -> Option<usize> {
        if self.board[row][col].tile == '-' {
            return None;
        }
        
        // get to the beginning of the word
        let mut start = row;
        while start > 0 && self.board[start-1][col].tile != '-' {
            start -= 1;
        }

        Some(start)
    }

    pub fn get_downmost_row(&self, row: usize, col: usize) -> Option<usize> {
        if self.board[row][col].tile == '-' {
            return None;
        }
        
        let mut end = row;
        while end < 14 && self.board[end+1][col].tile != '-' {
            end += 1;
        }

        Some(end)
    }

    pub fn submit(&mut self) -> i32 {
        // validate the proposed tile additions and submit them.
        // submission are valid if and only if:


        // special case if only one tile is submitted
        if self.staged_spaces.len() == 1 {
            let space = self.staged_spaces[0];
            let word = self.get_word_across(space.0, space.1);
            
            if !word.is_none() {
                let word = word.unwrap();
                match self.word_list.binary_search(&word) {
                    Ok(pos) => println!("{word} accepted"),
                    Err(E) => {println!("{word} not found in dictionary"); return 0 },
                }
            }
            
            let word = self.get_word_down(space.0, space.1);
            if !word.is_none() {
                let word = word.unwrap();
                match self.word_list.binary_search(&word) {
                    Ok(pos) => println!("{word} accepted"),
                    Err(E) => {println!("{word} not found in dictionary"); return 0 },
                }
            }
        }

        // 1. {
        // There is only one staged tile OR
        // All staged tiles have one fixed dimension (they go up/down or left/right) 
        //} AND 
        // 2. { All staged tiles abut either another staged tile or a committed tile } AND
        // 3. { All crossings between staged tiles and committed tiles form legal words }


        let mut const_row = false;
        let mut const_col = false;

        // validate fixed dimension:
        let tile1_row = self.staged_spaces[0].0;
        let tile1_col = self.staged_spaces[0].1;
        let tile2_row = self.staged_spaces[1].0;
        let tile2_col = self.staged_spaces[1].1;

        if tile1_row == tile2_row {
            const_row = true;
        }
        else if tile1_col == tile2_col {
            const_col = true;
        }
        else {
            println!("Invalid tile submission.");
            return 0;
        }

        for space in &self.staged_spaces {
            if const_row {
                if space.0 != tile1_row {
                    println!("Invalid tile submission. Non-constant row.");
                    return 0;
                }
            }
            else if const_col {
                if space.1 != tile1_col {
                    println!("Invalid tile submission. Non-constant column.");
                    return 0;
                }
            }
        }

        // validate contiguity, i.e. for each new tile, there is a path to another new tile along the fixed dimension.
        // one of the new tiles must come from the set of neighbor tiles (TODO later)

        // There will be a leftmost/upmost tile and a rightmost/downmost tile. First, we can find these and then confirm
        // that each space between the two is filled with a tile.


        // Get the max and min in the free dimension.
        let mut min_free = 16;
        let mut max_free = 0;
        if const_row {
            for space in &self.staged_spaces {
                if space.1 > max_free {
                    max_free = space.1;
                }
                if space.1 < min_free {
                    min_free = space.1;
                }
            }
        }
        else if const_col {
            for space in &self.staged_spaces {
                if space.0 > max_free {
                    max_free = space.1;
                }
                if space.0 < min_free {
                    min_free = space.1;
                }
            }
        }

        // check contiguity
        if const_row {
            let leftmost = self.get_leftmost_col(tile1_row, tile1_col).unwrap();
            let rightmost = self.get_rightmost_col(tile1_row, tile1_col).unwrap();


            for space in &self.staged_spaces {
                if space.1 > rightmost || space.1 < leftmost {
                    println!("Non-contiguous submission because {} not in [{} {}].", space.1, leftmost, rightmost);
                    return 0;
                }
            }

        }
        else if const_col {
            let upmost = self.get_upmost_row(tile1_row, tile1_col).unwrap();
            let downmost = self.get_downmost_row(tile1_row, tile1_col).unwrap();

            for space in &self.staged_spaces {
                if space.0 > downmost || space.0 < upmost {
                    println!("Non-contiguous submission because {} not in [{} {}].", space.0, upmost, downmost);
                    return 0;
                }
            }
        }

        // check the validity of the word
        if const_row {
            // check main word validity
            let main_word = self.get_word_across(tile1_row, tile1_col).unwrap();

            match self.word_list.binary_search(&main_word) {
                Ok(pos) => println!("{main_word} accepted"),
                Err(E) => {println!("{main_word} not found in dictionary"); return 0 },
            }

            // check crossing word validities
            for space in &self.staged_spaces {
                let word = self.get_word_down(space.0, space.1);

                if !word.is_none() {
                    let word = word.unwrap();
                    match self.word_list.binary_search(&word) {
                        Ok(pos) => println!("{word} accepted"),
                        Err(E) => {println!("{word} not found in dictionary"); return 0 },
                    }
                }
            }

        } else if const_col {
            let main_word = self.get_word_down(tile1_row, tile1_col).unwrap();
            match self.word_list.binary_search(&main_word) {
                Ok(word) => println!("{main_word} accepted"),
                Err(E) => {println!("{main_word} not found in dictionary"); return 0 },
            }

            // check crossing word validities
            for space in &self.staged_spaces {
                let word = self.get_word_across(space.0, space.1);

                if !word.is_none() {
                    let word = word.unwrap();
                    println!("Checking word {word}");
                    match self.word_list.binary_search(&word) {
                        Ok(pos) => println!("{word} accepted"),
                        Err(E) => {println!("{word} not found in dictionary"); return 0 },
                    }
                }
            }
        }

        // score staged word
        let score = self.score(const_row);
        println!("Play is worth {} points.", score);

        for space in &self.staged_spaces {
            self.board[space.0][space.1].val = score_letter(self.board[space.0][space.1].tile);
        }
        self.staged_spaces.clear();

        score
    }

    pub fn score(&self, across: bool) -> i32 {
        // accumulate the main score while keeping track of the crossing scores

        let mut word_mult = 1;
        let mut score = 0;
        let mut cross_score_sum = 0;
        let mut count = 0;

        for space in &self.staged_spaces {
            count += 1;
            word_mult *= self.board[space.0][space.1].word_mult;

            let tile_val = score_letter(self.board[space.0][space.1].tile);
            score += self.board[space.0][space.1].letter_mult * tile_val;

            println!("{} is worth {} for a running score of {}", self.board[space.0][space.1].tile, tile_val, score);

            let cross_score;
            if across {
                cross_score = self.score_down(space.0, space.1);
            }
            else {
                cross_score = self.score_across(space.0, space.1);
            }

            cross_score_sum += cross_score;
        }

        // get the remaining tiles already on the board
        if across {
            let origin = self.staged_spaces[0];
            let leftmost = self.get_leftmost_col(origin.0, origin.1).unwrap();
            let rightmost = self.get_rightmost_col(origin.0, origin.1).unwrap();

            for curr_col in leftmost..=rightmost {
                println!("Scoring {} {}", origin.0, curr_col);
                score += self.board[origin.0][curr_col].val;
            }
        } else {
            let origin = self.staged_spaces[0];
            let upmost = self.get_upmost_row(origin.0, origin.1).unwrap();
            let downmost = self.get_downmost_row(origin.0, origin.1).unwrap();

            for curr_row in upmost..=downmost {
                println!("Scoring {} {}", curr_row, origin.1);
                score += self.board[curr_row][origin.1].val;
            }
        }

        score *= word_mult;
        score += cross_score_sum;

        println!("cross_score_sum == {}", cross_score_sum);

        if count >= 7 {
            score += 50;
        }

        score
    }

    pub fn score_across(&self, row: usize, col: usize) -> i32 {
        let start_col = self.get_leftmost_col(row, col).unwrap();
        let end_col = self.get_rightmost_col(row, col).unwrap();
        
        println!("{start_col} {end_col}");

        if start_col == end_col {
            return 0;
        }

        let mut score = 0;
        for curr_col in start_col..=end_col {
            score += score_letter(self.board[row][curr_col].tile);
        }

        let word_mult = self.board[row][col].word_mult;
        let letter_mult = self.board[row][col].letter_mult;
        let crosser_score = score_letter(self.board[row][col].tile);

        score -= crosser_score;
        score += crosser_score * letter_mult;
        score *= word_mult;

        score
    }

    pub fn score_down(&self, row: usize, col: usize) -> i32 {
        let start_row = self.get_upmost_row(row, col).unwrap();
        let end_row = self.get_downmost_row(row, col).unwrap();

        if start_row == end_row {
            return 0;
        }

        let mut score = 0;
        for curr_row in start_row..=end_row {
            score += score_letter(self.board[curr_row][col].tile);
        }

        let word_mult = self.board[row][col].word_mult;
        let letter_mult = self.board[row][col].letter_mult;
        let crosser_score = score_letter(self.board[row][col].tile);

        score -= crosser_score;
        score += crosser_score * letter_mult;
        score *= word_mult;

        score
    }

    pub fn get_staged_bounds(&self, across: bool) -> (usize, usize) {
        let mut min = 15;
        let mut max = 0;
        if across {
            for space in &self.staged_spaces {
                if space.1 < min {
                    min = space.1;
                }
                if space.1 > max {
                    max = space.1;
                }
            }
        } else {
            for space in &self.staged_spaces {
                if space.0 < min {
                    min = space.0;
                }
                if space.0 > max {
                    max = space.0;
                }
            }
        }

        (min, max)
    }

    pub fn unstage(&mut self) {
        for space in &self.staged_spaces {
            self.board[space.0][space.1].tile = '-';
        }
        self.staged_spaces.clear();
    }

    pub fn show(&self) {
        let mut result = String::new();
        for row in &self.board {
            for space in row {
                if space.tile != '-' {
                    write!(&mut result, "{}  ", space.tile).unwrap();
                }
                else if space.letter_mult == 2 {
                    write!(&mut result, "dl ").unwrap();
                }
                else if space.letter_mult == 3 {
                    write!(&mut result, "tl ").unwrap();
                }
                else if space.word_mult == 2 {
                    write!(&mut result, "dw ").unwrap();
                }
                else if space.word_mult == 3 {
                    write!(&mut result, "tw ").unwrap();
                }
                else {
                    write!(&mut result, "-- ").unwrap();
                }
            }
            result.push('\n');
        }

        for c in &self.staged_spaces {
            let row = c.0;
            let col = 3*c.1 + 1;

            unsafe { // TODO: Make this safe later
                result.as_mut_vec()[row*46 + col] = b'+';
            }
        }

        println!("{result}");
    }
}

#[derive(Clone)]
pub struct Space {
    tile: char,
    letter_mult: i32,
    word_mult: i32,
    val: i32,
}

impl Space {
    pub fn set_char(&mut self, tile: char) {
        self.tile = tile;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let mut board = Board::new("dict.txt".to_string());

        board.put_tile('H', 7, 7);
        board.put_tile('E', 7, 8);
        board.put_tile('L', 7, 9);
        board.put_tile('L', 7, 10);
        board.put_tile('O', 7, 11);

        board.show();
    }

    #[test]
    fn test_fuzz() {
        // https://www.cross-tables.com/annotated.php?u=54918#3#

        let mut score;
        let mut board = Board::new("dict.txt".to_string());

        board.put_tile('J', 6, 7);
        board.put_tile('U', 7, 7);
        board.put_tile('B', 8, 7);
        board.put_tile('E', 9, 7);

        score = board.submit();
        assert_eq!(score, 26);

        board.put_tile('O', 8, 8);
        board.put_tile('X', 9, 8);

        score = board.submit();
        assert_eq!(score, 24);

        board.put_tile('T', 5, 8);
        board.put_tile('O', 6, 8);

        score = board.submit();
        assert_eq!(score, 13);

        board.put_tile('B', 3, 9);
        board.put_tile('R', 4, 9);
        board.put_tile('O', 5, 9);
        board.put_tile('W', 6, 9);

        score = board.submit();
        assert_eq!(score, 28);

        board.put_tile('H', 2, 10);
        board.put_tile('A', 3, 10);
        board.put_tile('E', 4, 10);
        board.put_tile('T', 5, 10);

        score = board.submit();
        assert_eq!(score, 25);

        board.put_tile('S', 7, 9);
        board.put_tile('A', 7, 10);
        board.put_tile('D', 7, 11);
        board.put_tile('D', 7, 12);
        board.put_tile('E', 7, 13);
        board.put_tile('N', 7, 14);

        score = board.submit();
        assert_eq!(score, 40);

        board.put_tile('T', 1, 11);
        board.put_tile('O', 2, 11);
        board.put_tile('G', 3, 11);

        score = board.submit();
        assert_eq!(score, 25);

        board.put_tile('L', 10, 1);
        board.put_tile('E', 10, 2);
        board.put_tile('F', 10, 3);
        board.put_tile('T', 10, 4);
        board.put_tile('I', 10, 5);
        board.put_tile('E', 10, 6);
        board.put_tile('S', 10, 7);

        score = board.submit();
        assert_eq!(score, 84);

        board.put_tile('I', 0, 13);
        board.put_tile('N', 1, 13);
        board.put_tile('T', 2, 13);
        board.put_tile('E', 3, 13);
        board.put_tile('R', 4, 13);
        board.put_tile('L', 5, 13);
        board.put_tile('I', 6, 13);

        score = board.submit();
        assert_eq!(score, 0); // invalid word
    }

    #[test]
    fn test_fuzz_2() {
        // https://www.cross-tables.com/annotated.php?u=54917#3#

        let mut score;
        let mut board = Board::new("dict.txt".to_string());

        board.put_tile('H', 6, 7);
        board.put_tile('I', 7, 7);
        board.put_tile('D', 8, 7);

        score = board.submit();
        assert_eq!(score, 14);

        board.put_tile('Z', 8, 2);
        board.put_tile('I', 8, 3);
        board.put_tile('N', 8, 4);
        board.put_tile('G', 8, 5);
        board.put_tile('E', 8, 6);

        score = board.submit();
        assert_eq!(score, 28);

        board.put_tile('I', 9, 2);
        board.put_tile('N', 10, 2);
        board.put_tile('N', 11, 2);
        board.put_tile('I', 12, 2);
        board.put_tile('A', 13, 2);

        score = board.submit();
        assert_eq!(score, 30);
        
        board.put_tile('J', 13, 1);
        board.put_tile('R', 13, 3);

        score = board.submit();
        assert_eq!(score, 20);

        board.put_tile('N', 10, 4);
        board.put_tile('E', 11, 4);
        board.put_tile('B', 12, 4);
        board.put_tile('S', 13, 4);

        score = board.submit();
        assert_eq!(score, 23);

        board.put_tile('W', 11, 0);
        board.put_tile('I', 11, 1);
        board.put_tile('T', 11, 3);
        board.put_tile('R', 11, 5);

        score = board.submit();
        assert_eq!(score, 26);

        board.put_tile('V', 4, 8);
        board.put_tile('U', 5, 8);
        board.put_tile('M', 6, 8);

        score = board.submit();
        assert_eq!(score, 21);

        board.put_tile('O', 1, 4);
        board.put_tile('U', 2, 4);
        board.put_tile('T', 3, 4);
        board.put_tile('E', 4, 4);
        board.put_tile('A', 5, 4);
        board.put_tile('T', 6, 4);
        board.put_tile('E', 7, 4);

        score = board.submit();
        assert_eq!(score, 66);

        board.put_tile('O', 0, 5);
        board.put_tile('X', 1, 5);

        score = board.submit();
        assert_eq!(score, 50);

        board.put_tile('O', 10, 5);
        board.put_tile('N', 10, 6);
        board.put_tile('D', 10, 7);
        board.put_tile('A', 10, 8);
        board.put_tile('I', 10, 9);
        board.put_tile('R', 10, 10);
        board.put_tile('Y', 10, 11);

        score = board.submit();
        assert_eq!(score, 76);

        board.put_tile('F', 0, 6);
        board.put_tile('F', 0, 7);
        board.put_tile('S', 0, 8);

        score = board.submit();
        assert_eq!(score, 30);

        board.put_tile('E', 11, 7);
        board.put_tile('C', 12, 7);
        board.put_tile('A', 13, 7);
        board.put_tile('L', 14, 7);

        score = board.submit();
        assert_eq!(score, 27);

        board.put_tile('M', 9, 11);
        board.put_tile('A', 9, 10);
        board.put_tile('H', 9, 9);
        
        score = board.submit();
        assert_eq!(score, 38);

        board.put_tile('S', 14, 2);
        board.put_tile('O', 14, 1);
        board.put_tile('C', 14, 0);

        score = board.submit();
        assert_eq!(score, 40);

        board.put_tile('T', 9, 3);
        board.put_tile('P', 7, 3);

        score = board.submit();
        assert_eq!(score, 17);

        board.put_tile('I', 14, 10);
        board.put_tile('E', 14, 14);
        board.put_tile('A', 14, 12);
        board.put_tile('G', 14, 11);
        board.put_tile('V', 14, 9);
        board.put_tile('T', 14, 13);
        board.put_tile('E', 14, 8);

        score = board.submit();
        assert_eq!(score, 92);

        board.put_tile('A', 13, 13);
        board.put_tile('Q', 12, 13);

        score = board.submit();
        assert_eq!(score, 24);

        board.put_tile('T', 5, 5);
        board.put_tile('N', 7, 5);
        board.put_tile('O', 6, 5);

        score = board.submit();
        assert_eq!(score, 18);

        board.put_tile('E', 8, 0);
        board.put_tile('B', 7, 0);
        board.put_tile('E', 10, 0);
        board.put_tile('D', 9, 0);

        score = board.submit();
        assert_eq!(score, 33);

        board.put_tile('S', 3, 7);
        board.put_tile('U', 3, 11);
        board.put_tile('I', 3, 9);
        board.put_tile('O', 3, 8);
        board.put_tile('L', 3, 10);
        board.put_tile('E', 3, 13);
        board.put_tile('R', 3, 12);

        score = board.submit();
        assert_eq!(score, 75);

        board.put_tile('A', 6, 1);
        board.put_tile('Y', 7, 1);
        board.put_tile('P', 5, 1);

        score = board.submit();
        assert_eq!(score, 21);

        board.put_tile('K', 1, 9);
        board.put_tile('O', 1, 8);
        
        score = board.submit();
        assert_eq!(score, 18);

        board.put_tile('W', 0, 14);
        board.put_tile('G', 2, 14);
        board.put_tile('S', 3, 14);
        board.put_tile('A', 1, 14);

        score = board.submit();
        assert_eq!(score, 36);

        board.put_tile('O', 2, 2);
        board.put_tile('D', 1, 2);
        board.put_tile('R', 4, 2);
        board.put_tile('U', 3, 2);
        board.put_tile('R', 6, 2);
        board.put_tile('E', 5, 2);

        score = board.submit();
        assert_eq!(score, 23);

        board.put_tile('I', 7, 12);
        board.put_tile('T', 6, 12);
        board.put_tile('E', 9, 12);
        board.put_tile('L', 8, 12);
        
        score = board.submit();
        assert_eq!(score, 15);
    }
}
