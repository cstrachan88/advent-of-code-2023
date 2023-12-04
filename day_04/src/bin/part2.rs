fn main() {
    let input = include_str!("../../../puzzle_inputs/day_04.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> u32 {
    let win_table = input.lines().map(get_wins).collect::<Vec<usize>>();
    let length = win_table.len();
    let mut card_counts = vec![1u32; length];

    for i in 0..length {
        // println!("Line {} Wins: {}", i + 1, win_table[i]);

        for j in 0..win_table[i] {
            let index = 1 + i + j;

            if index >= length {
                break;
            }
            card_counts[index] += card_counts[i];
        }
        // println!(" -- {} - {:?}", i, card_counts);
    }

    // println!("Card counts: {:?}", card_counts);
    card_counts.into_iter().sum::<u32>()
}

fn get_wins(line: &str) -> usize {
    let (_, numbers) = line.split_once(':').unwrap();
    let (winning_numbers, my_numbers) = numbers.split_once('|').unwrap();

    let winning_numbers = winning_numbers.trim().split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let my_numbers = my_numbers.trim().split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap());

    my_numbers.fold(0, |acc, n| {
        acc + if winning_numbers.contains(&n) {
            1
        } else {
            0
        }
    })
}

#[cfg(test)]
pub mod part1_tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#,
        );
        assert_eq!(result, 30);
    }
}
