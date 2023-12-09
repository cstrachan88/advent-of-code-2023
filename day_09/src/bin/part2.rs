fn main() {
    let input = include_str!("../../../puzzle_inputs/day_09.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &'static str) -> i32 {
    input.lines().map(|l| {
        let nums = l.as_bytes().split(|c| *c == b' ').map(parse_int).collect::<Vec<i32>>();

        println!("{:?}", nums);

        let result = nums[0] - search(&nums);
        
        println!("RESULT: {}", result);

        result
    }).sum()
}

fn search(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }

    let length = nums.len() - 1;
    let mut next_line: Vec<i32> = vec![0; length];

    let mut all_zeros = true;

    for i in 0..length {
        let left = nums[i];
        let right = nums[i + 1];
        let result = right - left;
        
        all_zeros &= result == 0;

        // println!(" -- Length: {}, i: {}, result: {}, all_zeros: {}", length, i, result, all_zeros);

        next_line[i] = result;
    }

    if all_zeros {
        return 0;
    }
    
    println!(" - {:?}", next_line);
    

    // (R) - L = (n)
    // L = R - n
    
    next_line[0] - search(&next_line)
}

fn parse_int(bytes: &[u8]) -> i32 {
    let mut result = 0i32;
    let negative = bytes[0] == b'-';
    
    let range: std::ops::Range<usize> = if negative {
        1..bytes.len()
    } else {
        0..bytes.len()
    };

    for i in range {
        result = 10 * result + (bytes[i] - b'0') as i32;
    }

    if negative { 0 - result } else { result }
}

#[cfg(test)]
pub mod part1_tests {
    use super::*;

    #[test]
    fn example() {
        let result = process(
            r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#,
        );
        assert_eq!(result, 2);
    }
}
