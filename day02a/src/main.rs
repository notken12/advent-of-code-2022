use std::{fs::File, env, io::{self, BufRead}};

#[derive(Debug)]
#[derive(PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn from_str(c: &str) -> Result<Self, ()> {
        match c {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }

    fn get_value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
            Self::Paper => Self::Rock,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).unwrap();

    let lines = io::BufReader::new(file).lines();

    let mut total_score = 0;

    for line in lines {
        if let Ok(line) = line {
            let opponent_play = Play::from_str(line.get(0..1).unwrap()).unwrap();
            let my_play = Play::from_str(line.get(2..3).unwrap()).unwrap();

            let outcome_score = if opponent_play == my_play {
                3
            } else {
                if my_play.beats() == opponent_play {
                    6
                } else {
                    0
                }
            };

            total_score += my_play.get_value() + outcome_score;
        }
    }

    println!("{total_score}");
}
