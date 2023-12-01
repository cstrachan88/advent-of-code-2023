fn main() {
    let input = include_str!("../../../puzzle_inputs/day_01.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(r#"part1"#);
        assert_eq!(result, "part1");
    }
}