fn main() {
    let input = include_str!("../../../puzzle_inputs/day_01.txt");
    let output = part1(input);
    dbg!(output);
}
// On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
// sum numbers
fn part1(input: &str) -> String {
    let calibration_values = input.lines().map(|l| {
        let mut digits = [' '; 2];
        let mut first_set = false;

        for c in l.chars() {
            if c.is_ascii_digit() {
                if !first_set {
                    digits[0] = c;
                    digits[1] = c;
                    first_set = true;
                    continue;
                }
                digits[1] = c;
            }
        }
        format!("{}{}", digits[0], digits[1])
            .parse::<u32>()
            .unwrap()
    });

    calibration_values.sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#,
        );
        assert_eq!(result, "142");
    }
}
