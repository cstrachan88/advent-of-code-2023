fn main() {
    let input = include_str!("../../../puzzle_inputs/day_02.txt");
    let output = part1(
        input,
        Set {
            red: 12,
            green: 13,
            blue: 14,
        },
    );
    dbg!(output);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn part1(input: &str, test_set: Set) -> u32 {
    input
        .lines()
        .map(|l| {
            let (_, game) = l.split_once(": ").unwrap();
            let mut sets = game.split("; ");

            let valid = sets.all(|p| {
                let mut parsed_set = Set::default();
                let cubes: Vec<&str> = p.split(", ").collect();

                for cube in cubes {
                    let space_index = cube.find(' ').unwrap();
                    let val = cube.get(..space_index).unwrap().parse::<u32>().unwrap();

                    match cube.as_bytes()[space_index + 1] {
                        b'r' => parsed_set.red += val,
                        b'g' => parsed_set.green += val,
                        b'b' => parsed_set.blue += val,
                        _ => (),
                    }
                }
                !(parsed_set.red > test_set.red
                    || parsed_set.green > test_set.green
                    || parsed_set.blue > test_set.blue)
            });

            if valid {
                // println!("PASS {}", l);

                let index_index = l.find(':').unwrap();
                l.get(5..index_index).unwrap().parse::<u32>().unwrap()
            } else {
                // println!("FAIL {}", l);
                0
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
            Set {
                red: 12,
                green: 13,
                blue: 14,
            },
        );
        assert_eq!(result, 8);
    }
}
