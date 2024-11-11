use std::str::FromStr;

pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Invalid input"),
        }
    }
}

pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err("Invalid input"),
        }
    }
}

impl Choice {
    pub fn points(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    pub fn match_point(&self, other: &Choice) -> usize {
        match (self, other) {
            (Choice::Rock, Choice::Rock) => 3 + self.points(),
            (Choice::Rock, Choice::Paper) => 0 + self.points(),
            (Choice::Rock, Choice::Scissors) => 6 + self.points(),
            (Choice::Paper, Choice::Rock) => 6 + self.points(),
            (Choice::Paper, Choice::Paper) => 3 + self.points(),
            (Choice::Paper, Choice::Scissors) => 0 + self.points(),
            (Choice::Scissors, Choice::Rock) => 0 + self.points(),
            (Choice::Scissors, Choice::Paper) => 6 + self.points(),
            (Choice::Scissors, Choice::Scissors) => 3 + self.points(),
        }
    }

    pub fn predict(&self, other: &Outcome) -> usize {
        let my_choice = match (self, other) {
            (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Rock, Outcome::Lose) => Choice::Scissors,
            (Choice::Rock, Outcome::Draw) => Choice::Rock,
            (Choice::Paper, Outcome::Win) => Choice::Scissors,
            (Choice::Paper, Outcome::Lose) => Choice::Rock,
            (Choice::Paper, Outcome::Draw) => Choice::Paper,
            (Choice::Scissors, Outcome::Win) => Choice::Rock,
            (Choice::Scissors, Outcome::Lose) => Choice::Paper,
            (Choice::Scissors, Outcome::Draw) => Choice::Scissors,
        };
        my_choice.match_point(&self)
    }
}
