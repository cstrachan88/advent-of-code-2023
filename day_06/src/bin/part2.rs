fn main() {
    let input = include_str!("../../../puzzle_inputs/day_06.txt");
    let output = process(input);
    dbg!(output);
}

// dist:       333,163,512,891,532
// time input:          53,827,288
// (.5 * t)^2: 724,344,233,358,736
// u32::max:         4,294,967,295
// use u64

// Use binary search since number is large

pub fn process(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = parse_line(lines.next().unwrap());
    let dist = parse_line(lines.next().unwrap());

    let mut left = 0;
    let mut right = time / 2;

    // println!("Race Distance: {dist}, Race Time: {time}");

    while left <= right {
        let mid = (left + right) / 2;
        let calculated = calc_dist(time, mid);
        
        // println!(" -- left: {left}, right: {right}, mid: {mid}, calculated: {calculated}");

        if calculated < dist {
            left = mid + 1;
        } else if calculated > dist {
            right = mid - 1;
        } else {
            break;
        }
    }
    // Use right value to ensure we are greater than or equal to calculated distance
    // Subtract one for even to adjust for duplicate value
    if time & 1 == 1 {
        2 * (time / 2 - right)
    } else {
        2 * (time / 2 - right) - 1
    }
}

fn parse_line(line: &str) -> u64 {
    let (_, nums) = line.split_once(':').unwrap();
    parse_int(nums.split_ascii_whitespace().collect::<String>().as_str())
}

fn parse_int(s: &str) -> u64 {
    let mut result = 0u64;
    for b in s.as_bytes() {
        result = 10 * result + (b - b'0') as u64;
    }
    result
}

fn calc_dist(t_total: u64, t_held: u64) -> u64 {
    (t_total - t_held) * t_held
}

#[cfg(test)]
pub mod part1_tests {
    use super::*;
    
    #[test]
    fn example() {
        let result = process(
            r#"Time:      7  15   30
Distance:  9  40  200"#,
        );
        assert_eq!(result, 71503);
    }
    
    #[test]
    fn example1() {
        let result = process(
            r#"Time:      7
Distance:  9"#,
        );
        assert_eq!(result, 4);
    }
    
    #[test]
    fn example2() {
        let result = process(
            r#"Time:      15  
Distance:   40  "#,
        );
        assert_eq!(result, 8);
    }
    
    #[test]
    fn example3() {
        let result = process(
            r#"Time:      30
Distance:  200"#,
        );
        assert_eq!(result, 9);
    }
}
