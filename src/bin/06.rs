advent_of_code::solution!(6);
use itertools::Itertools;

fn quatratic_formula(time: u64, distance: u64) -> u64 {
    // distance = x * (time - x)
    // distance = time * x - x^2
    // x^2 - time * x + distance = 0
    // x = (time +- sqrt(time^2 - 4 * distance)) / 2
    
    let a = ((time * time - 4 * distance) as f32).sqrt();
    let x1 = ((time as f32 - a) / 2.0 + 1.0).floor();
    let x2 = ((time as f32 + a) / 2.0 - 1.0).ceil();
    (x2 - x1 + 1.0) as u64
}

pub fn part_one(input: &str) -> Option<u32> {
    let (time, distance) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|s| s.parse::<u64>().unwrap())
        })
        .next_tuple()
        .unwrap();

    time.zip(distance)
        .map(|(t, d)| quatratic_formula(t, d))
        .product::<u64>()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (time, distance) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .next_tuple()
        .unwrap();

    quatratic_formula(time, distance).try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
