fn main() {
    let input = include_str!("../../../puzzle_inputs/day_03.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> u32 {
    let mut part_number_sum = 0;
    // processing as bytes since rust uses unicode ([u8; 4]) chars
    let line_bytes: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let length = line_bytes.len();

    // Can symbols be letters? Apparently not

    for i in 0..length {
        // println!("{:?}", line_bytes[i]);
        // unsafe {
        //     println!("{}", std::str::from_utf8_unchecked(line_bytes[i]));
        // }

        let line_length: usize = line_bytes[0].len();
        let mut start: usize = line_length;
        let mut stop: usize = line_length;

        let mut finding: bool;

        for j in 0..line_length {
            if line_bytes[i][j].is_ascii_digit() {
                finding = true;

                if start == line_length {
                    start = j;
                }
                stop = j;
            } else {
                finding = false;
            }
            if j == line_length - 1 {
                finding = false;
            }

            if !finding && start != line_length {
                // found a complete number, so check for symbols
                // symbols are on:
                // -- prev_line.get(start - 1..=stop + 1)
                // -- current line[start - 1], current_line[stop + 1]
                // -- next_line.get(start - 1..=stop + 1)

                unsafe {
                    let mut found_symbol = false;

                    let check_start = i32::max(0, start as i32 - 1) as usize;
                    let check_stop = i32::min((line_length - 1) as i32, stop as i32 + 1) as usize;

                    if i > 0 {
                        found_symbol = found_symbol
                            || line_bytes[i - 1]
                                .get_unchecked(check_start..=check_stop)
                                .iter()
                                .any(|b| *b != b'.' && b.is_ascii_punctuation());
                    }
                    found_symbol = found_symbol
                        || (line_bytes[i][check_start] != b'.'
                            && line_bytes[i][check_start].is_ascii_punctuation());
                    found_symbol = found_symbol
                        || (line_bytes[i][check_stop] != b'.'
                            && line_bytes[i][check_stop].is_ascii_punctuation());
                    if i < line_length - 1 {
                        found_symbol = found_symbol
                            || line_bytes[i + 1]
                                .get_unchecked(check_start..=check_stop)
                                .iter()
                                .any(|b| *b != b'.' && b.is_ascii_punctuation());
                    }

                    if found_symbol {
                        let exponents = (0..=(stop - start) as u32).rev();
                        let num = line_bytes[i]
                            .get_unchecked(start..=stop)
                            .iter()
                            .zip(exponents)
                            .map(|(b, pow)| {
                                let digit: u32 = (b - b'0') as u32;
                                let base: u32 = 10;
                                digit * base.pow(pow)
                            })
                            .sum::<u32>();

                        // println!("  num: {}", num);
                        part_number_sum += num;
                    }
                }

                start = line_length;
                stop = line_length;
            }
        }
        // println!("  SUM SO FAR: {}", part_number_sum);
    }
    part_number_sum
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
        assert_eq!(result, 4361);
    }

    #[test]
    fn end_of_line() {
        let result = process(
            r#"..*11
...*2
...*3
...*4"#,
        );
        assert_eq!(result, 20);
    }
}
