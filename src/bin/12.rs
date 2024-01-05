advent_of_code::solution!(12);
use itertools::Itertools;
use memoize::memoize;
#[memoize]
fn calculate_permutations(sequence: String, order: Vec<u64>) -> u64 {
    if sequence.len() == 0 {
        if order.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if order.len() == 0 {
        if sequence.chars().any(|c| c == '#') {
            return 0;
        } else {
            return 1;
        }
    }

    let mut res = 0;
    if let Some(first_char) = sequence.chars().nth(0) {
        if first_char == '.' || first_char == '?' {
            res += calculate_permutations(sequence[1..].to_string(), order.clone());
        }
        if first_char == '#' || first_char == '?' {
            if order[0] <= sequence.len() as u64
                && sequence[..order[0] as usize].chars().all(|c| c != '.')
                && (order[0] == sequence.len() as u64
                    || sequence.chars().nth(order[0] as usize).unwrap() != '#')
            {
                let next_sequence = if order[0] == sequence.len() as u64 {
                    ""
                } else {
                    &sequence[order[0] as usize + 1..]
                };
                res += calculate_permutations(next_sequence.to_string(), order[1..].to_vec());
            }
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    input.lines().fold(Some(0), |acc, line| {
        let (sequence, order_str) = line.split_once(' ').unwrap();
        let order = order_str
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        acc.and_then(|acc| Some(acc + calculate_permutations(sequence.to_string(), order)))
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    input.lines().fold(Some(0), |acc, line| {
        let (sequence, order_str) = line.split_once(' ').unwrap();
        let sequence = (0..5).map(|_| sequence).join("?");
        let order_str = (0..5).map(|_| order_str).join(",");

        let order = order_str
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        acc.and_then(|acc| Some(acc + calculate_permutations(sequence, order)))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
