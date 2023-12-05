fn main() {
    let input = include_str!("../../../puzzle_inputs/day_05.txt");
    let output = process(input);
    dbg!(output);
}

/*
almanac (your puzzle input) lists all of the seeds that need to be planted.
It also lists what type of soil to use with each kind of seed, what type of
fertilizer to use with each kind of soil, what type of water to use with
each kind of fertilizer, and so on. Every type of seed, soil, fertilizer
and so on is identified with a number, but numbers are reused by each
category - that is, soil 123 and fertilizer 123 aren't necessarily related
to each other.

almanac starts by listing which seeds need to be planted
rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category
Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted
Each line within a map contains three numbers: the destination range start, the source range start, and the range length

destination range start of 50, a source range start of 98, and a range length of 2
source range starts at 98 and contains two values: 98 and 99
destination range is the same length, but it starts at 50, so its two values are 50 and 51
seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51

Any source numbers that aren't mapped correspond to the same destination number
seed number 10 corresponds to soil number 10

like to know the closest location that needs a seed
find the lowest location number that corresponds to any of the initial seeds

seeds
seed-to-soil
soil-to-fertilizer
fertilizer-to-water
water-to-light
light-to-temperature
temperature-to-humidity
humidity-to-location

*/

pub fn process(input: &str) -> u64 {
    let mut lines = input.lines();

    let (_, seeds) = lines.next().unwrap().split_once(": ").unwrap();

    let mut input = seeds.split(' ').map(parse_int).collect::<Vec<_>>();
    let mut output = input.clone();

    let seed_count = input.len();
    
    lines.next();
    for line in lines {
        if line.is_empty() {
            println!("   Input: {:?}", input);
            println!("   Output: {:?}", output);

            input = output.clone();

            continue;
        }
        if !line.as_bytes()[0].is_ascii_digit() {
            continue;
        }

        let map = line.split(' ').map(parse_int).collect::<Vec<_>>();
        let (dest, src, range) = (map[0], map[1], map[2]);

        for i in 0..seed_count {
            if input[i] >= src && input[i] < src + range {
                output[i] = dest + (input[i] - src)
            }
        }

        println!("{}", line);
    }
    *output.iter().min().unwrap()
}

fn parse_int(s: &str) -> u64 {
    let mut result = 0u64;
    for b in s.as_bytes() {
        result = 10 * result + (b - b'0') as u64;
    }
    result
}

#[cfg(test)]
pub mod part1_tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#,
        );
        assert_eq!(result, 35);
    }
}
