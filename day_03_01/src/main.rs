use std::collections::HashSet;

fn item_priority(item: char) -> usize {
    match item {
        _ if item.is_ascii_lowercase() => item as usize - ('a' as usize) + 1,
        _ if item.is_ascii_uppercase() => item as usize - ('A' as usize) + 27,
        _ => 0,
    }
}

fn first_common_item(rucksack: &str) -> Option<char> {
    let (first, second) = rucksack.split_at(rucksack.len() / 2);

    let first: HashSet<char> = first.chars().collect();
    let second: HashSet<char> = second.chars().collect();

    first.intersection(&second).next().map(|item| item.clone())
}

fn main() {
    let log = include_str!("../input.txt");

    let priorities: usize = log
        .split("\n")
        .map(first_common_item)
        .filter_map(|item| item.map(|item| item_priority(item)))
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
        assert!(first_common_item("") == None);
        assert!(first_common_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL") == Some('L'));
        assert!(first_common_item("dqSCqVWdbDSSVqbVVSqhNdrnpnCnfsnnwfnsRpMpBMrf") == Some('C'));
    }
}
