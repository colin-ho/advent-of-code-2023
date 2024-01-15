advent_of_code::solution!(13);

fn find_reflection(pattern: Vec<Vec<u8>>) -> Option<usize> {
    'outer: for i in 0..pattern.len() - 1 {
        if pattern[i] == pattern[i + 1] {
            let min_distance = i.min(pattern.len() - i - 2);
            for d in 1..=min_distance {
                if pattern[i - d] != pattern[i + d + 1] {
                    continue 'outer;
                }
            }
            return Some(i);
        }
    }
    None
}

fn find_reflection_part2(pattern: Vec<Vec<u8>>) -> Option<usize> {
    'outer: for i in 0..pattern.len() - 1 {
        let mut diff = pattern[i]
            .iter()
            .zip(pattern[i + 1].iter())
            .filter(|(a, b)| a != b)
            .count();

        if diff <= 1 {
            let min_distance = i.min(pattern.len() - i - 2);
            for d in 1..=min_distance {
                diff += pattern[i - d]
                    .iter()
                    .zip(pattern[i + d + 1].iter())
                    .filter(|(a, b)| a != b)
                    .count();
                if diff > 1 {
                    continue 'outer;
                }
            }
            if diff == 0 {
                continue 'outer;
            }
            return Some(i);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .fold(0, |acc, group| {
            let grid: Vec<Vec<u8>> = group
                .split('\n')
                .map(|row| row.as_bytes().to_vec())
                .collect::<Vec<_>>();
            let grid_transpose = (0..grid[0].len())
                .map(|i| grid.iter().map(|row| row[i]).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let mut res = 0;
            if let Some(horizontal_reflection) = find_reflection(grid) {
                res += (horizontal_reflection + 1) * 100;
            }
            if let Some(vertical_reflection) = find_reflection(grid_transpose) {
                res += vertical_reflection + 1;
            }
            acc + res
        })
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .fold(0, |acc, group| {
            let grid: Vec<Vec<u8>> = group
                .split('\n')
                .map(|row| row.as_bytes().to_vec())
                .collect::<Vec<_>>();
            let grid_transpose = (0..grid[0].len())
                .map(|i| grid.iter().map(|row| row[i]).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let mut res = 0;
            if let Some(horizontal_reflection) = find_reflection_part2(grid) {
                res += (horizontal_reflection + 1) * 100;
            }
            if let Some(vertical_reflection) = find_reflection_part2(grid_transpose) {
                res += vertical_reflection + 1;
            }
            acc + res
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
