#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<&str> for Shape {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err("Bad input! Expected 'A', 'B', or 'C'"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl TryFrom<&str> for Outcome {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("Bad input! Expected 'X', 'Y', or 'Z'"),
        }
    }
}

impl Shape {
    fn play(self, other: Shape) -> Outcome {
        match other {
            _ if other == self.wins_against() => Outcome::Win,
            _ if other == self.loses_against() => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }

    fn loses_against(self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn wins_against(self) -> Shape {
        self.loses_against().loses_against()
    }

    fn guess_other(self, outcome: Outcome) -> Shape {
        match outcome {
            Outcome::Win => self.loses_against(),
            Outcome::Draw => self,
            Outcome::Lose => self.wins_against(),
        }
    }
}

fn score_from_line(line: &str) -> Option<usize> {
    let mut items = line.split(" ");
    let opponent = items.next().and_then(|value| Shape::try_from(value).ok())?;
    let outcome = items
        .next()
        .and_then(|value| Outcome::try_from(value).ok())?;
    let own = opponent.guess_other(outcome);
    Some(own.play(opponent) as usize + own as usize)
}

fn main() {
    let log = include_str!("../../day_02_01/input.txt");

    let score = log
        .split("\n")
        .map(score_from_line)
        .fold(0, |sum, round_score| sum + round_score.unwrap_or(0));

    println!("{}", score);
}
