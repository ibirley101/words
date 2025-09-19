use crate::game::{Bag, Board, Rack};
use crate::greedy::find_greediest_word;
use crate::player::Player;

enum ShellCommand {
    Exit,
    Help(Vec<String>),
    Put(Vec<String>),
    Show,
    Submit,
    Swap(Vec<String>),
    Unstage,
    WriteAcross(Vec<String>),
    WriteDown(Vec<String>),
    Err, // Catchall
}

pub enum ShellStatus {
    Continue,
    Exit,
    Submit(i32),
    Swap,
    Err,
}

pub struct Shell<'a> {
    bag: &'a mut Bag,
    board: &'a mut Board,
    player: &'a mut Player,
}

impl<'a> Shell<'a> {
    pub fn new(bag: &'a mut Bag, board: &'a mut Board, player: &'a mut Player) -> Self {
        Shell {
            bag: bag,
            board: board,
            player: player,
        }
    }

    pub fn main_loop(&mut self) -> ShellStatus {
        loop {
            print!("> ");
            let line = self.read_line();
            let cmd = self.parse(line);
            let status = self.execute(cmd);

            match status {
                ShellStatus::Continue => continue,
                ShellStatus::Exit => return status,
                ShellStatus::Submit(_) => return status,
                ShellStatus::Swap => return status,
                ShellStatus::Err => continue,
            }
        }
    }

    fn read_line(&self) -> String {
        use std::io::{Write, stdin, stdout};
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        s
    }

    fn parse(&self, line: String) -> ShellCommand {
        let cmd;
        let mut args = Vec::new();

        let mut iter = line.split_whitespace();
        cmd = match iter.next() {
            Some(s) => s.to_string(),
            None => String::new(),
        };

        while let Some(s) = iter.next() {
            args.push(s.to_string());
        }

        match cmd.as_str() {
            "exit" => ShellCommand::Exit,
            "help" => ShellCommand::Help(args),
            "put" => ShellCommand::Put(args),
            "show" => ShellCommand::Show,
            "submit" => ShellCommand::Submit,
            "swap" => ShellCommand::Swap(args),
            "unstage" => ShellCommand::Unstage,
            "wa" => ShellCommand::WriteAcross(args),
            "wd" => ShellCommand::WriteDown(args),
            _ => ShellCommand::Err,
        }
    }

    fn parse_put(&self, mut args: Vec<String>) -> Option<(char, usize, usize)> {
        let letter;
        let row;
        let col;
        if args.len() != 3 {
            println!("Error: Syntax: put [LETTER] [ROW_INDEX] [COLUMN_INDEX]");
            return None;
        }
        col = args.pop().expect("Vector is length 3.");
        row = args.pop().expect("Vector is length 3.");
        letter = args.pop().expect("Vector is length 3.");

        if letter.len() != 1 {
            println!("Error: More than one letter provided.");
            return None;
        }
        let letter: char = letter.chars().nth(0).expect("Letter is length 1.");
        if !letter.is_alphabetic() {
            println!("Error: Syntax: put [LETTER] [ROW_INDEX] [COLUMN_INDEX]");
            return None;
        }
        let row: usize = match row.parse() {
            Ok(n) => {
                if n > 14 {
                    println!("Row out of bounds.");
                    return None;
                } else {
                    n
                }
            }
            Err(_) => return None,
        };

        let col: usize = match col.parse() {
            Ok(n) => {
                if n > 14 {
                    println!("Col out of bounds.");
                    return None;
                } else {
                    n
                }
            }
            Err(_) => return None,
        };

        Some((letter, row, col))
    }

    fn parse_write(&self, mut args: Vec<String>) -> Option<(String, usize, usize)> {
        let word;
        let row;
        let col;
        if args.len() != 3 {
            println!("Error: Syntax: put [WORD] [ROW_INDEX] [COLUMN_INDEX]");
            return None;
        }
        col = args.pop().expect("Vector is length 3.");
        row = args.pop().expect("Vector is length 3.");
        word = args.pop().expect("Vector is length 3.");

        for letter in word.chars() {
            if !letter.is_alphabetic() {
                println!("Error: Non-alphabetic letter detected.");
                return None;
            }
        }
        let row: usize = match row.parse() {
            Ok(n) => {
                if n > 14 {
                    println!("Row out of bounds.");
                    return None;
                } else {
                    n
                }
            }
            Err(_) => return None,
        };

        let col: usize = match col.parse() {
            Ok(n) => {
                if n > 14 {
                    println!("Col out of bounds.");
                    return None;
                } else {
                    n
                }
            }
            Err(_) => return None,
        };

        Some((word, row, col))
    }

    fn execute(&mut self, cmd: ShellCommand) -> ShellStatus {
        match cmd {
            ShellCommand::Exit => ShellStatus::Exit,
            ShellCommand::Help(args) => self.exec_help(args),
            ShellCommand::Put(args) => self.exec_put(args),
            ShellCommand::Show => self.exec_show(),
            ShellCommand::Submit => self.exec_submit(),
            ShellCommand::Swap(args) => self.exec_swap(args),
            ShellCommand::Unstage => self.exec_unstage(),
            ShellCommand::WriteAcross(args) => self.exec_write_across(args),
            ShellCommand::WriteDown(args) => self.exec_write_down(args),
            ShellCommand::Err => ShellStatus::Err,
        }
    }

    fn exec_help(&mut self, args: Vec<String>) -> ShellStatus {
        if !self.player.rackless {
            if args.len() != 0 {
                println!("Unexpected arguments.");
                return ShellStatus::Err;
            }

            let (word, score, row, col, across) =
                find_greediest_word(&mut self.board, &self.player.rack);
            if across {
                println!("Highest scorer is {word} at ({row}, {col}) across for {score} points.");
            } else {
                println!("Highest scorer is {word} at ({row}, {col}) down for {score} points.");
            }
        } else {
            if args.len() != 1 {
                println!("Please provide all chars with no delimiters.");
                return ShellStatus::Err;
            }

            let helpstr = args.first().expect("Checked that first exists.").chars();
            let mut help_rack = Rack::new();
            for c in helpstr {
                help_rack.add_tile(c);
            }
            if help_rack.is_empty() {
                return ShellStatus::Err;
            }
            let (word, score, row, col, across) = find_greediest_word(self.board, &help_rack);
            if across {
                println!("Highest scorer is {word} at ({row}, {col}) across for {score} points.");
            } else {
                println!("Highest scorer is {word} at ({row}, {col}) down for {score} points.");
            }
        }
        ShellStatus::Continue
    }

    fn exec_put(&mut self, args: Vec<String>) -> ShellStatus {
        let (letter, row, col) = match self.parse_put(args) {
            Some((letter, row, col)) => (letter, row, col),
            None => return ShellStatus::Err,
        };
        self.board.put_tile(letter, row, col);

        ShellStatus::Continue
    }

    fn exec_show(&self) -> ShellStatus {
        self.board.show();
        self.player.rack.show();
        ShellStatus::Continue
    }

    fn exec_submit(&mut self) -> ShellStatus {
        let score_delta = self.board.submit();
        if score_delta == 0 {
            ShellStatus::Continue
        } else {
            ShellStatus::Submit(score_delta)
        }
    }

    fn exec_swap(&mut self, args: Vec<String>) -> ShellStatus {
        if args.len() != 1 {
            println!("Please provide all chars with no delimiters.");
            return ShellStatus::Err;
        }

        let swapstr = args.first().expect("Checked that first exists.").chars();
        let mut to_swap = Vec::new();
        for c in swapstr {
            to_swap.push(c.to_ascii_uppercase());
        }
        
        if self.player.rack.swap(&mut self.bag, to_swap) {
            ShellStatus::Swap
        }
        else {
            ShellStatus::Err
        }
    }

    fn exec_unstage(&mut self) -> ShellStatus {
        if self.player.rackless {
            self.board.unstage();
        } else {
            self.board.unstage_to_rack(&mut self.player.rack);
        }
        ShellStatus::Continue
    }

    fn exec_write_across(&mut self, args: Vec<String>) -> ShellStatus {
        let (word, row, col) = match self.parse_write(args) {
            Some((word, row, col)) => (word, row, col),
            None => return ShellStatus::Err,
        };
        self.board
            .write_across_from_rack(&mut self.player.rack, word, row, col);

        ShellStatus::Continue
    }

    fn exec_write_down(&mut self, args: Vec<String>) -> ShellStatus {
        let (word, row, col) = match self.parse_write(args) {
            Some((word, row, col)) => (word, row, col),
            None => return ShellStatus::Err,
        };
        self.board
            .write_down_from_rack(&mut self.player.rack, word, row, col);

        ShellStatus::Continue
    }
}
