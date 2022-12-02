#[derive(PartialEq, Eq, Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn parse_shape(input: &str) -> Option<Shape> {
    match input {
        "A" => Some(Shape::Rock),
        "B" => Some(Shape::Paper),
        "C" => Some(Shape::Scissors),
        _ => None,
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

fn parse_outcome(input: &str) -> Option<Outcome> {
    match input {
        "X" => Some(Outcome::Lose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None,
    }
}

fn parse_line(input: &str) -> Option<(Shape, Outcome)> {
    let mut items = input.split(" ");
    match (
        items.next().map(parse_shape).unwrap_or(None),
        items.next().map(parse_outcome).unwrap_or(None),
    ) {
        (Some(opponent), Some(outcome)) => Some((opponent, outcome)),
        _ => None,
    }
}

fn guess_own(opponent: Shape, outcome: Outcome) -> Shape {
    match (opponent, outcome) {
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        (_, Outcome::Draw) => opponent,
    }
}

fn round_score(own: Shape, outcome: Outcome) -> usize {
    own as usize + outcome as usize
}

fn main() {
    let log = include_str!("../../day_02_01/input.txt");

    let score = log
        .split("\n")
        .map(parse_line)
        .fold(0, |running_score, round| {
            running_score + round.map_or(0, |(opponent, outcome)| round_score(guess_own(opponent, outcome), outcome))
        });

    println!("{}", score);
}
