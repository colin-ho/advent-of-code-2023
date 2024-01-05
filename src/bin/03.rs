advent_of_code::solution!(3);
use std::cmp::min;

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let cell = grid[i][j];
            if cell != '.' && !cell.is_numeric() {
                for ii in i.saturating_sub(1)..=min(i + 1, grid.len() - 1) {
                    for jj in j.saturating_sub(1)..=min(j + 1, grid[i].len() - 1) {
                        if ii == i && jj == j {
                            continue;
                        }
                        if grid[ii][jj].is_numeric() {
                            let mut r = jj;
                            while r < grid[ii].len() && grid[ii][r].is_numeric() {
                                r += 1;
                            }
                            let mut l = jj;
                            while l > 0 && grid[ii][l - 1].is_numeric() {
                                l -= 1;
                            }
                            let number = grid[ii][l..r].iter().collect::<String>();
                            res += number.parse::<u32>().unwrap();

                            for k in l..r {
                                grid[ii][k] = '.';
                            }
                        }
                    }
                }
            }
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let cell = grid[i][j];
            if cell == '*' {
                let mut parts = Vec::new();
                for ii in i.saturating_sub(1)..=min(i + 1, grid.len() - 1) {
                    for jj in j.saturating_sub(1)..=min(j + 1, grid[i].len() - 1) {
                        if ii == i && jj == j {
                            continue;
                        }
                        if grid[ii][jj].is_numeric() {
                            let mut r = jj;
                            while r < grid[ii].len() && grid[ii][r].is_numeric() {
                                r += 1;
                            }
                            let mut l = jj;
                            while l > 0 && grid[ii][l - 1].is_numeric() {
                                l -= 1;
                            }
                            let number = grid[ii][l..r].iter().collect::<String>();
                            parts.push(number.parse::<u32>().unwrap());

                            for k in l..r {
                                grid[ii][k] = '.';
                            }
                        }
                    }
                }

                if parts.len() == 2 {
                    res += parts[0] * parts[1];
                }
            }
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
