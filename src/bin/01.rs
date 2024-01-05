advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let res = input.lines().fold(0, |acc, line| {
        let mut digits = line.chars().filter_map(|x| x.to_digit(10));
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        acc + first * 10 + last
    });
    Some(res)
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    let res = input.lines().fold(0, |acc, line| {
        let mut first = None;
        let mut last = None;

        let mut set_digits = |d: u32| {
            if first.is_none() {
                first = Some(d);
            }
            last = Some(d);
        };
        
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                let digit = c.to_digit(10).unwrap();
                set_digits(digit);
            } else {
                for (j, d) in DIGITS.iter().enumerate() {
                    if line[i..].starts_with(d) {
                        set_digits((j + 1) as u32);
                    }
                }
            }
        }

        acc + first.unwrap() * 10 + last.unwrap()
    });
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
