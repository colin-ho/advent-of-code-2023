use num::integer::Integer;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, data) = input.split_once("\n\n").unwrap();
    let map = data
        .lines()
        .map(|line| {
            let (source, dest) = line.split_once(" = ").unwrap();
            let (left, right) = dest
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();
            (source, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut node = "AAA";
    let mut count = 0;
    for instruction in instructions.chars().cycle() {
        if node == "ZZZ" {
            break;
        }
        let (left, right) = map.get(node)?;
        node = match instruction {
            'L' => left,
            'R' => right,
            _ => return None,
        };
        count += 1;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, data) = input.split_once("\n\n").unwrap();
    let map = data
        .lines()
        .map(|line| {
            let (source, dest) = line.split_once(" = ").unwrap();
            let (left, right) = dest
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();
            (source, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut res: u64 = 1;
    for &start in map.keys().filter(|key| key.ends_with('A')) {
        let mut node = start;
        let mut count = 0;
        for instruction in instructions.chars().cycle() {
            if node.ends_with('Z') {
                break;
            }
            let (left, right) = map.get(node)?;
            node = match instruction {
                'L' => left,
                'R' => right,
                _ => return None,
            };
            count += 1;
        }
        res = res.lcm(&count);
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
