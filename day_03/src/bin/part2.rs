fn main() {
    let input = include_str!("../../../puzzle_inputs/day_03.txt");
    let output = process(input);
    dbg!(output);
}

// A gear is any * symbol that is adjacent to exactly two part numbers
// gear ratio is the result of multiplying those two numbers together
// find the gear ratio of every gear and add them all up

pub fn process(input: &str) -> u32 {
    // processing as bytes since rust uses unicode ([u8; 4]) chars
    let line_bytes: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let length = line_bytes.len();

    let mut sum_gear_ratios = 0;

    for i in 0..length {
        // unsafe {
        //     println!("{}", std::str::from_utf8_unchecked(line_bytes[i]));
        // }
        let line_length: usize = line_bytes[0].len();

        for j in 0..line_length {
            if line_bytes[i][j] == b'*' {
                // println!(" -- FOUND POTENTIAL GEAR -- ");
                let mut gears: Vec<u32> = Vec::new();

                // Checking previous line
                if i > 0 {
                    get_adjacent_line_numbers(line_bytes[i - 1], j, &mut gears);
                }

                // Checking next line
                if i < length - 1 {
                    get_adjacent_line_numbers(line_bytes[i + 1], j, &mut gears);
                }

                // Checking current line - left
                if j > 0 && line_bytes[i][j - 1].is_ascii_digit() {
                    let range = find_range(line_bytes[i], j, 0, Direction::Left);
                    unsafe {
                        gears.push(
                            std::str::from_utf8_unchecked(
                                line_bytes[i].get_unchecked(range.1..=range.0),
                            )
                            .parse::<u32>()
                            .unwrap(),
                        );
                    }
                }

                // Checking current line - right
                if j < line_length - 1 && line_bytes[i][j + 1].is_ascii_digit() {
                    let range = find_range(line_bytes[i], j, line_length - 1, Direction::Right);
                    unsafe {
                        gears.push(
                            std::str::from_utf8_unchecked(
                                line_bytes[i].get_unchecked(range.0..=range.1),
                            )
                            .parse::<u32>()
                            .unwrap(),
                        );
                    }
                }

                // Get rid of gear with more than 2 parts
                // println!(" Gears: {:?}", gears);
                if gears.len() == 2 {
                    sum_gear_ratios += gears.into_iter().product::<u32>();
                }
            }
        }
    }
    sum_gear_ratios
}

fn get_adjacent_line_numbers(line: &[u8], index: usize, gears: &mut Vec<u32>) {
    let line_length = line.len();

    if line[index].is_ascii_digit() {
        // Grab above number
        let range1 = find_range(line, index, 0, Direction::Left);
        let range2 = find_range(line, index, line_length - 1, Direction::Right);
        unsafe {
            gears.push(
                std::str::from_utf8_unchecked(line.get_unchecked(range1.1..=range2.1))
                    .parse::<u32>()
                    .unwrap(),
            );
        }
    } else {
        // Grab above/below-left number
        if index > 0 && line[index - 1].is_ascii_digit() {
            let range = find_range(line, index, 0, Direction::Left);
            unsafe {
                gears.push(
                    std::str::from_utf8_unchecked(line.get_unchecked(range.1..=range.0))
                        .parse::<u32>()
                        .unwrap(),
                );
            }
        }
        //  Grab above/below-right number
        if index < line_length - 1 && line[index + 1].is_ascii_digit() {
            let range = find_range(line, index, line_length - 1, Direction::Right);
            unsafe {
                gears.push(
                    std::str::from_utf8_unchecked(line.get_unchecked(range.0..=range.1))
                        .parse::<u32>()
                        .unwrap(),
                );
            }
        }
    }
}

enum Direction {
    Left,
    Right,
}

fn find_range(line: &[u8], index: usize, bounds: usize, direction: Direction) -> (usize, usize) {
    let range_part: usize;
    let increment: i32;
    let mut found_index = index as i32;

    match direction {
        Direction::Left => {
            range_part = index - 1;
            increment = -1;
        }
        Direction::Right => {
            range_part = index + 1;
            increment = 1;
        }
    }

    while line[(found_index + increment) as usize].is_ascii_digit() {
        found_index += increment;
        if found_index as usize == bounds {
            break;
        }
    }

    (range_part, found_index as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let result = process(
            r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
        );
        assert_eq!(result, 467835);
    }

    #[test]
    fn testing_found_index1() {
        let result = process(
            r#"100
.*.
200"#,
        );
        assert_eq!(result, 20000);
    }

    #[test]
    fn testing_found_index2() {
        let result = process(
            r#"100.200
...*..."#,
        );
        assert_eq!(result, 20000);
    }

    #[test]
    fn testing_found_index3() {
        let result = process(
            r#"1.200
.*..."#,
        );
        assert_eq!(result, 200);
    }

    #[test]
    fn testing_found_index4() {
        let result = process(
            r#"1.2
.*."#,
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn testing_found_index5() {
        let result = process(
            r#"100
.*.
..1"#,
        );
        assert_eq!(result, 100);
    }

    #[test]
    fn testing_found_index6() {
        let result = process(
            r#"100....
...*....
....200"#,
        );
        assert_eq!(result, 20000);
    }

    #[test]
    fn testing_found_index7() {
        let result = process(r#"100*200"#);
        assert_eq!(result, 20000);
    }
}
