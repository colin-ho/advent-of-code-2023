advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().fold(0, |acc, line| {
        let (_, nums) = line.split_once(": ").unwrap();
        let (winning_str, mine_str) = nums.split_once(" | ").unwrap();

        let winning_nums = winning_str
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let sum = mine_str
            .split_whitespace()
            .filter_map(|num| {
                let num = num.parse::<u32>().unwrap();
                if winning_nums.contains(&num) {
                    Some(1)
                } else {
                    None
                }
            })
            .sum::<u32>();

        if sum > 0 {
            acc + (1 << (sum - 1))
        } else {
            acc
        }
    }).try_into().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut copies: Vec<u32> = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let (_, nums) = line.split_once(": ").unwrap();
        let (winning_str, mine_str) = nums.split_once(" | ").unwrap();

        let winning_nums = winning_str
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let sum = mine_str
            .split_whitespace()
            .filter_map(|num| {
                let num = num.parse::<u32>().unwrap();
                if winning_nums.contains(&num) {
                    Some(1)
                } else {
                    None
                }
            })
            .sum::<usize>();
        
        for j in (i + 1)..(i + sum + 1) {
            copies[j] += copies[i];
        }
    }

    Some(copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
