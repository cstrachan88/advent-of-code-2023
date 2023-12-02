fn main() {
    let input = include_str!("../../../puzzle_inputs/day_02.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            // println!("{}", l);

            let (_, game) = l.split_once(": ").unwrap();
            let sets = game.split("; ");
            let mut max_set = Set::default();

            for set in sets {
                let cubes: Vec<&str> = set.split(", ").collect();

                for cube in cubes {
                    let space_index = cube.find(' ').unwrap();
                    let val = cube.get(..space_index).unwrap().parse::<u32>().unwrap();

                    match cube.as_bytes()[space_index + 1] {
                        b'r' => max_set.red = u32::max(max_set.red, val),
                        b'g' => max_set.green = u32::max(max_set.green, val),
                        b'b' => max_set.blue = u32::max(max_set.blue, val),
                        _ => (),
                    }
                }
            }
            max_set.red * max_set.green * max_set.blue
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
        );
        assert_eq!(result, 2286);
    }
}
