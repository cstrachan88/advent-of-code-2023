fn main() {
    let input = include_str!("../../../puzzle_inputs/day_07.txt");
    let output = process(input);
    dbg!(output);
}

// Is there a way in rust to have a const number
// I want to have a const HAND_SIZE of 11
// I can use this in functions, but if I wanted to set it as the array size parameter for the parse_line fn,
// how would I do that?

use std::cmp::Ordering;
use std::collections::HashMap;

// Cards are ordered by
// Five of a kind -> four of a kind -> full house -> three of a kind -> two pair -> one pair -> high card
// Second ordering is by going card by card
// Ordering: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J
// J is now Joker which is wildcard = the card that makes the best hand

pub fn process(input: &str) -> u32 {
    let mut cards = input
        .lines()
        .map(parse_line)
        .collect::<Vec<([u8; 11], u32)>>();

    // sorts smallest to largest
    cards.sort_by(|a, b| {
        for i in 0..11 {
            let compare = a.0[i].cmp(&b.0[i]);
            if compare != Ordering::Equal {
                return compare;
            }
        }
        Ordering::Equal
    });

    let mut index = 0;
    let mut total_winnings = 0;

    for card in cards {
        index += 1;
        let winnings = index * card.1;
        total_winnings += winnings;
        println!(
            "{:>4} - Hand: {:2?}, Bid: {:>3} - Winnings: {:>6}",
            index, card.0, card.1, winnings
        );
    }
    total_winnings
}

fn parse_line(line: &str) -> ([u8; 11], u32) {
    let bytes = line.as_bytes();

    let mut hand = [0u8; 11];
    let mut index = 6;

    for b in unsafe { bytes.get_unchecked(0..5) }
        .iter()
        .map(|b| match b {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' => 1,
            b'T' => 10,
            _ => b - b'0',
        })
    {
        hand[index] = b;
        index += 1;
    }

    let unique_cards =
        unsafe { hand.get_unchecked(6..11) }
            .iter()
            .fold(HashMap::new(), |mut acc, c| {
                acc.entry(c).and_modify(|x| *x += 1).or_insert(1u8);
                acc
            });

    let unique_count = unique_cards.len();

    if unique_count == 1 {
        hand[0] = 1; // 5 of kind
    } else if unique_count == 2 {
        // With Joker: AAAAJ, AAAJJ, AAJJJ, AJJJJ - 5 of kind
        // W/o Joker : AAAAB - 4 of kind / AAABB - full house
        if unique_cards.contains_key(&1) {
            hand[0] = 1; // 5 of kind
        } else {
            let check_card_count = unique_cards[&hand[6]];

            match check_card_count {
                4 | 1 => hand[1] = 1, // 4 of kind
                _ => hand[2] = 1,     // full house
            }
        }
    } else if unique_count == 3 {
        // With Joker: AAABJ, AABJJ, ABJJJ - 4 of kind / AABBJ - full house
        // W/o Joker : AAABC - 3 of kind / AABBC - 2 pair
        if unique_cards.contains_key(&1) {
            let joker_count = unique_cards[&1];

            if joker_count > 1 {
                hand[1] = 1; // 4 of kind
            } else {
                for count in unique_cards.values() {
                    if *count == 3 {
                        hand[1] = 1; // 4 of kind
                        break;
                    } else if *count == 2 {
                        hand[2] = 1; // full house
                        break;
                    }
                }
            }
        } else {
            let highest_count = *unique_cards.values().max().unwrap();

            if highest_count == 3 {
                hand[3] = 1; // 3 of kind
            } else if highest_count == 2 {
                hand[4] = 1; // 2 pair
            }
        }
    } else if unique_count == 4 {
        // With Joker: AABCJ, ABCJJ - 3 of kind
        // W/o Joker : AABCD - 1 pair
        if unique_cards.contains_key(&1) {
            hand[3] = 1; // 3 of kind
        } else {
            hand[5] = 1; // 1 pair
        }
    } else if unique_count == 5 && unique_cards.contains_key(&1) {
        // Joker makes a pair
        hand[5] = 1; // 1 pair
    }

    (hand, parse_int(unsafe { bytes.get_unchecked(6..) }))
}

fn parse_int(bytes: &[u8]) -> u32 {
    let mut result = 0u32;
    for b in bytes {
        result = 10 * result + (b - b'0') as u32;
    }
    result
}

#[cfg(test)]
pub mod part1_tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#,
        );
        assert_eq!(result, 5905);
    }
}
