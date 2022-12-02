#[derive(PartialEq, Eq, Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn parse_shape(input: &str) -> Option<Shape> {
    match input {
        "A" | "X" => Some(Shape::Rock),
        "B" | "Y" => Some(Shape::Paper),
        "C" | "Z" => Some(Shape::Scissors),
        _ => None,
    }
}

fn parse_line(input: &str) -> Option<(Shape, Shape)> {
    let mut shapes = input.split(" ").map(parse_shape);
    match (shapes.next(), shapes.next()) {
        (Some(Some(opponent)), Some(Some(own))) => Some((own, opponent)),
        _ => None,
    }
}

fn match_score(own: Shape, opponent: Shape) -> usize {
    match (own, opponent) {
        | (Shape::Rock, Shape::Scissors)
        | (Shape::Scissors, Shape::Paper)
        | (Shape::Paper, Shape::Rock) => 6,
        _ if own == opponent => 3,
        _ => 0,
    }
}

fn round_score(own: Shape, opponent: Shape) -> usize {
    own as usize + match_score(own, opponent)
}

fn main() {
    let log = include_str!("../input.txt");

    let score = log
        .split("\n")
        .map(parse_line)
        .fold(0, |running_score, round| {
            running_score + round.map_or(0, |(own, opponent)| round_score(own, opponent))
        });

    println!("{}", score);
}
