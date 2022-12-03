use std::collections::HashSet;

fn item_priority(item: char) -> usize {
    match item {
        _ if item.is_ascii_lowercase() => item as usize - ('a' as usize) + 1,
        _ if item.is_ascii_uppercase() => item as usize - ('A' as usize) + 27,
        _ => 0,
    }
}

fn first_common_item(rucksacks: &[&str]) -> Option<char> {
    let mut sets = rucksacks.iter().map(|rucksack| rucksack.chars().collect());

    let intersection: Option<HashSet<char>> = sets
        .next()
        .map(|set| sets.fold(set, |set1, set2| &set1 & &set2));

    intersection.and_then(|it| it.iter().next().cloned())
}

fn main() {
    let log = include_str!("../../day_03_01/input.txt");

    let priorities: usize = log
        .split('\n')
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(first_common_item)
        .map(|item| item.map(item_priority).unwrap_or(0))
        .sum();

    println!("{}", priorities);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_priority_works() {
        assert!(item_priority('a') == 1);
        assert!(item_priority('z') == 26);
        assert!(item_priority('A') == 27);
        assert!(item_priority('Z') == 52);
        assert!(item_priority('?') == 0);
        assert!(item_priority('🐹') == 0);
    }

    #[test]
    fn common_item_works() {
        assert!(
            first_common_item(&[
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
            ]) == Some('r')
        );

        assert!(
            first_common_item(&[
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]) == Some('Z')
        );
    }
}
