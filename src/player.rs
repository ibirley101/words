use crate::{
    game::{Bag, Board, Rack},
    greedy::find_greediest_word,
    shell::{Shell, ShellStatus},
};

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

        let turn_result;
        if self.cpu {
            turn_result = self.play_turn_cpu(board, bag);
        } else {
            turn_result = self.play_turn_player(board, bag);
        }

        let score_delta = match turn_result {
            TurnResult::Score(n) => n,
            TurnResult::Swap => 0,
            TurnResult::Exit => return TurnResult::Exit,
        };

        self.score += score_delta;
        println!("Score: {}\n", self.score);
        TurnResult::Score(score_delta)
    }

    fn play_turn_cpu(&mut self, board: &mut Board, _bag: &mut Bag) -> TurnResult {
        let word_choice = find_greediest_word(board, &mut self.rack);
        if word_choice.across {
            board.write_across_from_rack(&mut self.rack, word_choice.word, word_choice.row, word_choice.col);
        } else {
            board.write_down_from_rack(&mut self.rack, word_choice.word, word_choice.row, word_choice.col);
        }
        let score = board.submit();

        TurnResult::Score(score)
    }

    fn play_turn_player(&mut self, board: &mut Board, bag: &mut Bag) -> TurnResult {
        let mut shell = Shell::new(bag, board, self);
        match shell.main_loop() {
            ShellStatus::Exit => TurnResult::Exit,
            ShellStatus::Submit(score) => TurnResult::Score(score),
            ShellStatus::Swap => TurnResult::Swap,
            _ => TurnResult::Exit, // should not be possible
        }
    }
}
