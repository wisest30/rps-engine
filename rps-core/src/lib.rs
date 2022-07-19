use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[repr(u8)]
#[derive(FromPrimitive)]
pub enum Choice {
    Rock = 0,
    Paper,
    Scissors,
}

struct Score {
    win: usize,
    draw: usize,
    lose: usize,
}

impl Score {
    fn new() -> Self {
        Score {
            win: 0,
            draw: 0,
            lose: 0,
        }
    }

    fn add_win(&mut self) {
        self.win += 1;
    }

    fn add_draw(&mut self) {
        self.draw += 1;
    }

    fn add_lose(&mut self) {
        self.lose += 1;
    }

    fn get_win(&self) -> usize {
        self.win
    }

    fn get_draw(&self) -> usize {
        self.draw
    }

    fn get_lose(&self) -> usize {
        self.lose
    }
}

pub struct Game {
    score: Score,
    enemy_choice: Option<Choice>,
    last_result: String,
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: Score::new(),
            enemy_choice: None,
            last_result: "".to_string(),
        }
    }

    pub fn play(&mut self, choice: Choice) {
        let choice_idx = choice as u8;
        self.enemy_choice = Some(Choice::Rock);

        let enemy_choice_idx = rand::random::<u8>() % 3;
        self.enemy_choice = Some(u8_to_choice(enemy_choice_idx));

        self.last_result = if (choice_idx + 1) % 3 == enemy_choice_idx {
            self.score.add_lose();
            "you lose".to_string()
        } else if choice_idx == enemy_choice_idx {
            self.score.add_draw();
            "draw".to_string()
        } else {
            self.score.add_win();
            "you win".to_string()
        }
    }

    pub fn make_score_board(&self) -> String {
        format!(
            "win: {}, draw: {}, lose: {}\nenemy's last choice: {}\n last result: {}",
            self.score.get_win(),
            self.score.get_draw(),
            self.score.get_lose(),
            if let Some(choice) = &self.enemy_choice {
                match choice {
                    Choice::Rock => "Rock",
                    Choice::Paper => "Paper",
                    Choice::Scissors => "Scissors",
                }
            } else {
                "none"
            },
            self.last_result
        )
    }
}

pub fn u8_to_choice(i: u8) -> Choice {
    FromPrimitive::from_u8(i).expect("Chice enum matching failed")
}

pub fn test_str() -> String {
    "testtest".to_string()
}
