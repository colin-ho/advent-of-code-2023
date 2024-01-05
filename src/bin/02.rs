advent_of_code::solution!(2);
use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .fold(0, |acc, line| {
            let (id, game) = line.split_once(": ").unwrap();
            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;
            for round in game.split("; ") {
                for cube in round.split(", ") {
                    let mut iter = cube.split_whitespace();
                    let amount = iter.next().unwrap().parse::<u32>().unwrap();
                    let color = iter.next().unwrap();

                    match color {
                        "blue" => blue += amount,
                        "red" => red += amount,
                        "green" => green += amount,
                        _ => panic!("unknown color"),
                    }
                }
            }
            if red <= 12 && green <= 13 && blue <= 14 {
                acc + id
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            } else {
                acc
            }
        })
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .fold(0, |acc, line| {
            let game = line.split(": ").nth(1).unwrap();
            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;
            for round in game.split("; ") {
                for cube in round.split(", ") {
                    let mut iter = cube.split_whitespace();
                    let amount = iter.next().unwrap().parse::<u32>().unwrap();
                    let color = iter.next().unwrap();

                    match color {
                        "blue" => blue = max(blue, amount),
                        "red" => red = max(red, amount),
                        "green" => green = max(green, amount),
                        _ => panic!("unknown color"),
                    }
                }
            }

            acc + red * green * blue
        })
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
