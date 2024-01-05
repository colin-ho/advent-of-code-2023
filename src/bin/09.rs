advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .fold(0, |acc, line| {
            let mut values = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let mut ends = Vec::new();
            loop {
                // add last value of values to ends
                ends.push(values[values.len() - 1]);
                // set values equal to the differences between each element in values
                values = values.windows(2).map(|w| w[1] - w[0]).collect();

                if values.iter().all(|&x| x == 0) {
                    break;
                }
            }
            // return sum of ends
            acc + ends.iter().sum::<i64>()
        })
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .fold(0, |acc, line| {
            let mut values = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .rev()
                .collect::<Vec<_>>();
            let mut ends = Vec::new();
            loop {
                // add last value of values to ends
                ends.push(values[values.len() - 1]);
                // set values equal to the differences between each element in values
                values = values.windows(2).map(|w| w[1] - w[0]).collect();

                if values.iter().all(|&x| x == 0) {
                    break;
                }
            }
            // return sum of ends
            acc + ends.iter().sum::<i64>()
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
