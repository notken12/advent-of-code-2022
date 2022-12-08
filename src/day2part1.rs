#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

const BEATS: (Play, Play, Play) = (Play::Rock, Play::Paper, Play::Scissors);

impl Play {
    fn from_char(c: &char) -> Result<Self, ()> {
        match c {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
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
}

fn main() {
    println!("{:?}", Play::from_char(&'B'));
    println!("{:?}", Play::from_char(&'D'));
    println!("{:?}", Play::from_char(&'Z').unwrap().get_value());
}
