fn main() {
    let input = include_str!("../../../puzzle_inputs/day_01.txt");
    let output = part2(input);
    dbg!(output);
}
// Do I need to worry about upper or lowercase? -- Apparently not
// Any zeros to ignore -- Apparently not
// Also numbers seem to overlap -- since only valid combos occur on last letter of prev number,
// return index of last letter instead of index after parsed digit

// Goes char by char through the string, if it finds a digit, return it,
// if it finds a letter that starts a number, check the rest of the number's letters
// Also modifies index so parent function knows where to start next
fn find_digit(bytes: &[u8], index: &mut usize) -> u8 {
    if *index >= bytes.len() {
        return 0;
    }

    // Index is advanced in every match expression - do it once here instead
    *index += 1;

    // If numerical digit is found, return it
    if bytes[*index - 1].is_ascii_digit() && bytes[*index - 1] != b'0' {
        bytes[*index - 1]
    } else {
        match bytes[*index - 1] {
            // b'z' => {
            //     if bytes.get(*index..*index + 3) == Some(&[b'e', b'r', b'o']) {
            //         *index += 2;
            //         return b'0';
            //     }
            // },
            b'o' => {
                if bytes.get(*index..*index + 2) == Some(&[b'n', b'e']) {
                    *index += 1;
                    return b'1';
                }
            }
            b't' => {
                if bytes.get(*index..*index + 2) == Some(&[b'w', b'o']) {
                    *index += 1;
                    return b'2';
                }
                if bytes.get(*index..*index + 4) == Some(&[b'h', b'r', b'e', b'e']) {
                    *index += 3;
                    return b'3';
                }
            }
            b'f' => {
                if bytes.get(*index..*index + 3) == Some(&[b'o', b'u', b'r']) {
                    *index += 2;
                    return b'4';
                }
                if bytes.get(*index..*index + 3) == Some(&[b'i', b'v', b'e']) {
                    *index += 2;
                    return b'5';
                }
            }
            b's' => {
                if bytes.get(*index..*index + 2) == Some(&[b'i', b'x']) {
                    *index += 1;
                    return b'6';
                }
                if bytes.get(*index..*index + 4) == Some(&[b'e', b'v', b'e', b'n']) {
                    *index += 3;
                    return b'7';
                }
            }
            b'e' => {
                if bytes.get(*index..*index + 4) == Some(&[b'i', b'g', b'h', b't']) {
                    *index += 3;
                    return b'8';
                }
            }
            b'n' => {
                if bytes.get(*index..*index + 3) == Some(&[b'i', b'n', b'e']) {
                    *index += 2;
                    return b'9';
                }
            }
            _ => (),
        }
        find_digit(bytes, index)
    }
}

// On each line, the calibration value can be found by combining the first digit and the last digit
// (in that order) to form a single two-digit number.
// Digits can be spelled out in letters
// Result is sum of all calibration values
fn part2(input: &str) -> String {
    let calibration_values = input.lines().map(|l| {
        // Since I know the input file is valid ascii, I shouldn't use string, but bytes
        // See: https://stackoverflow.com/a/67308339/5231317
        let bytes = l.as_bytes();
        let mut digits = [b'0'; 2];

        let mut index: usize = 0;
        let mut digit = find_digit(bytes, &mut index);

        digits[0] = digit;
        digits[1] = digit;

        loop {
            digit = find_digit(bytes, &mut index);

            if digit == 0 {
                break;
            } else {
                digits[1] = digit;
            }
        }

        // Digits can be retrieved by converting from ascii byte to num when subtracting '0' offset
        // (48 on ascii chart)
        let val = (10 * (digits[0] - b'0') + digits[1] - b'0') as u32;
        println!("{}: {}", val, l);
        val
    });

    calibration_values.sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#,
        );
        assert_eq!(result, "281");
    }
}
