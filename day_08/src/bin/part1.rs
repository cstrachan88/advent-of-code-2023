fn main() {
    let input = include_str!("../../../puzzle_inputs/day_08.txt");
    let output = process(input);
    dbg!(output);
}

use std::collections::HashMap;

pub fn process(input: &str) -> u32 {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().as_bytes();
    let instruction_length = instructions.len();
    let mut index = 0;

    lines.next();
    let nodes = lines.map(parse_line).collect::<HashMap<&str, (&str, &str)>>();

    let mut current = "AAA";

    loop {
        let next = if instructions[index % instruction_length] == b'L' {
            nodes[current].0
        } else {
            nodes[current].1
        };
        
        println!("Current: {}, Next: {}", current, next);

        if next == "ZZZ" {
            break;
        }

        current = next;
        index += 1;
    }

    1 + index as u32
}

fn parse_line(line: &str) -> (&str, (&str, &str)) {
    unsafe {
        (line.get_unchecked(0..3), (line.get_unchecked(7..10), line.get_unchecked(12..15)))
    }
}

#[cfg(test)]
pub mod part1_tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#,
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn example2() {
        let result = process(
            r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#,
        );
        assert_eq!(result, 6);
    }
}
