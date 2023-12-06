fn main() {
    let input = include_str!("../../../puzzle_inputs/day_06.txt");
    let output = process(input);
    dbg!(output);
}

// Simple kinematic equation
// x = v * t
// v = time held
// t = time remaining

pub fn process(input: &str) -> u32 {
    let mut lines = input.lines();
    let times = parse_line(lines.next().unwrap());
    let dists = parse_line(lines.next().unwrap());
    let count = times.len();
    
    let mut num_wins = 1;

    for i in 0..count {
        let mid = times[i] / 2;

        for t in 0..times[i] {
            println!("{:?} - t_rem: {}, dist: {}", (times[i], dists[i]), t, calc_dist(times[i], t));

            if calc_dist(times[i], t) > dists[i] {
                if times[i] & 1 == 1 { // odd
                    num_wins *= 2 * (1 + mid - t);
                } else { // even
                    // num_wins *= 2 * (1 + mid - t) - 1;
                    num_wins *= 2 * (mid - t) + 1;
                }
                
                println!("num_wins: {}, mid: {}, t: {}", num_wins, mid, t);
                break;
            }
        }
    }    
    num_wins
}

fn parse_line(line: &str) -> Vec<u32> {
    let (_, nums) = line.split_once(':').unwrap();
    nums.split_ascii_whitespace().map(parse_int).collect::<Vec<u32>>()
}

fn parse_int(s: &str) -> u32 {
    let mut result = 0u32;
    for b in s.as_bytes() {
        result = 10 * result + (b - b'0') as u32;
    }
    result
}

fn calc_dist(t_total: u32, t_held: u32) -> u32 {
    (t_total - t_held) * t_held
}

#[cfg(test)]
pub mod part1_tests {
    use super::*;

    #[test]
    fn dist_calc() {
        let time_vs_dist = [(0, 0), (1, 6), (2, 10), (3, 12), (4, 12), (5, 10), (6, 6), (7, 0)];
        for check in time_vs_dist {
            assert_eq!(check.1, calc_dist(7, check.0));
        }
    }

    #[test]
    fn example() {
        let result = process(
            r#"Time:      7  15   30
Distance:  9  40  200"#,
        );
        assert_eq!(result, 288);
    }
}
