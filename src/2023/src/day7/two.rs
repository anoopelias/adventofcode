use std::{cmp::Ordering, collections::HashMap};

use crate::utils::parser::SeparatorParser;

struct Hand {
    cards: String,
    bid: i32,
    ty: usize,
}

pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut hands = vec![];
    for line in lines {
        let splits = line.parse_separator(" ");
        let cards = splits.get(0).unwrap();
        let bid = splits.get(1).unwrap().parse::<i32>().unwrap();
        hands.push(Hand {
            cards: cards.to_string(),
            bid,
            ty: type_of(cards),
        });
    }

    hands.sort_by(|hand_a, hand_b| compare(hand_a, hand_b));

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        let winnings = ((i + 1) as i32) * hand.bid;
        sum += winnings;
    }
    sum.to_string()
}

fn compare(hand_a: &Hand, hand_b: &Hand) -> Ordering {
    if hand_a.ty != hand_b.ty {
        return hand_a.ty.cmp(&hand_b.ty);
    }

    let chars_a: Vec<char> = hand_a.cards.chars().collect();
    let chars_b: Vec<char> = hand_b.cards.chars().collect();
    for i in 0..5 {
        let char_a = chars_a.get(i).unwrap();
        let char_b = chars_b.get(i).unwrap();
        if char_a != char_b {
            return to_num(*char_a).cmp(&to_num(*char_b));
        }
    }

    Ordering::Equal
}

fn type_of(cards: &str) -> usize {
    let mut card_set = HashMap::new();
    cards.chars().for_each(|card| {
        if !card_set.contains_key(&card) {
            card_set.insert(card, 0);
        }
        let n = card_set.get(&card).unwrap();
        card_set.insert(card, n + 1);
    });

    let js = card_set.remove(&'J').unwrap_or(0);
    if card_set.is_empty() {
        return 6;
    }
    let mut keys = card_set.keys().collect::<Vec<&char>>();
    keys.sort_by(|ka, kb| card_set.get(ka).unwrap().cmp(card_set.get(kb).unwrap()));
    let key = keys.last().unwrap();
    card_set.insert(**key, card_set.get(&key).unwrap() + js);

    match card_set.keys().len() {
        5 => 0,                                                           // High card
        4 => 1,                                                           // One pair
        3 if card_set.values().collect::<Vec<&i32>>().contains(&&2) => 2, // Two pairs
        3 => 3,                                                           // Three of a kind
        2 if card_set.values().collect::<Vec<&i32>>().contains(&&2) => 4, // Full house
        2 => 5,                                                           // Four of a kind
        1 => 6,                                                           // Five of a kind
        _ => panic!("unexpected"),
    }
}

fn to_num(ch: char) -> u32 {
    if ch.is_numeric() {
        ch.to_digit(10).unwrap()
    } else {
        match ch {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            'J' => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{day7::two::solve, utils::util};

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day7/input");
        assert_eq!("5905", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day7/input1");
        assert_eq!("249781879", solve(lines))
    }
}
