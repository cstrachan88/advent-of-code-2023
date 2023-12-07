fn main() {
    let input = include_str!("../../../puzzle_inputs/day_07.txt");
    let output = process(input);
    dbg!(output);
}

use std::collections::HashMap;
use std::cmp::Ordering;

// Cards are ordered by
// Five of a kind -> four of a kind -> full house -> three of a kind -> two pair -> one pair -> high card
// Second ordering is by going card by card
// instead of calculating or whatever, just prepend the hand with the value of the win type
// FfHTtOH XXXXX, where the first section is the value of the card of the win type, and the second section is the bid
// 1234567 89012 -> turn card into 12 byte array
// Ordering:  A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2

// In the case of the following hands:
// AA8AA and AAA3A
// According to the rules, the second hand is better than the first

// Be careful of the full house
// "77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger"
// Shouldn't the 77888 be stronger since it has a higher value?
// Apparently not.

// Don't prepend values in hand array, just prepend ones (or bools?) if it falls into category

pub fn process(input: &str) -> u32 {
    let mut cards = input.lines().map(parse_line).collect::<Vec<([u8; 12], u32)>>();

    // sorts smallest to largest
    cards.sort_by(|a, b| {
        for i in 0..12 {
            let compare = a.0[i].cmp(&b.0[i]);
            if compare != Ordering::Equal {
                return compare;
            }
        }
        Ordering::Equal
    });

    // cards.into_iter().enumerate().map(|(i, (_, bid))| {
    //     (i as u32 + 1) * bid
    // }).sum()

    let mut index = 0;
    let mut total_winnings = 0;

    for card in cards {
        index += 1;
        let winnings = index * card.1;
        total_winnings += winnings;
        println!("{:>4} - Hand: {:2?}, Bid: {:>3} - Winnings: {:>6}", index, card.0, card.1, winnings);
    }
    total_winnings
}

fn parse_line(line: &str) -> ([u8; 12], u32) {
    let bytes = line.as_bytes();

    let mut hand = [0u8; 12];
    let mut index = 7;

    for b in unsafe { bytes.get_unchecked(0..5) }.iter().map(|b| {
        match b {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' => 11,
            b'T' => 10,
            _ => b - b'0'
        }
    }) {
        hand[index] = b;
        index += 1;
    }
    // dbg!(hand);
    
    let unique_cards = unsafe { hand.get_unchecked(7..12) }.iter().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|x| *x += 1).or_insert(1u8);
        acc
    });
    // dbg!(unique_card_count);

    let unique_count = unique_cards.len();
    
    if unique_count == 1 {
        // Five of a kind
        hand[0] = 1;
    } else if unique_count == 2 {
        let check_card_count = unique_cards[&hand[7]];

        match check_card_count {
            // Four of a kind
            4 => hand[1] = 1,
            1 => hand[1] = 1,

            // Full House
            3 => hand[2] = 1,
            _ => hand[2] = 1,
        }
    } else if unique_count == 3 {
        let mut highest_count = 0u8;

        for (_, count) in unique_cards {
            if count > highest_count {
                highest_count = count;
            }
        }

        if highest_count == 3 {
            // Three of a kind
            hand[3] = 1;
        } else if highest_count == 2 {
            // Two pair
            hand[4] = 1;
        }
    } else if unique_count == 4 {
        // One pair
        hand[5] = 1;
    } else {
        // Highest card
        hand[6] = 1;
    }
    
    // if unique_count == 1 {
    //     // Five of a kind
    //     hand[0] = hand[7];
    // } else if unique_count == 2 {
    //     let check_card_count = unique_cards[&hand[7]];
    //     match check_card_count {
    //         // Four of a kind
    //         4 => hand[1] = hand[7],
    //         1 => hand[1] = hand[8],

    //         // Full House
    //         3 => hand[2] = hand[7],
    //         _ => {
    //             for i in 8..12 {
    //                 if hand[i] != hand[7] {
    //                     hand[2] = hand[i];
    //                     break;
    //                 }
    //             }
    //         },
    //     }
    // } else if unique_count == 3 {
    //     let mut highest_count = 0u8;
    //     let mut the_card = 0u8;

    //     for (card, count) in unique_cards {
    //         if count > highest_count {
    //             highest_count = count;
    //             the_card = *card;
    //         } else if count == highest_count && *card > the_card {
    //             the_card = *card;
    //         }
    //     }

    //     if highest_count == 3 {
    //         // Three of a kind
    //         hand[3] = the_card;
    //     } else if highest_count == 2 {
    //         // Two pair
    //         hand[4] = the_card;
    //     }
    // } else if unique_count == 4 {
    //     // One pair
    //     for (card, count) in unique_cards {
    //         if count == 2 {
    //             hand[5] = *card;
    //             break;
    //         }
    //     }
    // } else {
    //     // Highest card
    //     let mut largest = 0u8;
    //     (7..12).for_each(|i| {
    //         if hand[i] > largest {
    //             largest = hand[i];
    //         }
    //     });
    //     hand[6] = largest;
    // }
    
    let bid = parse_int(unsafe { bytes.get_unchecked(6..) });

    // println!("Hand: {:?}, Bid: {}", hand, bid);
    (hand, bid)
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
        assert_eq!(result, 6440);
    }
}
