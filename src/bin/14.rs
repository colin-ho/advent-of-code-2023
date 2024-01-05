advent_of_code::solution!(14);

fn move_up(grid: &mut Vec<Vec<u8>>, mut row: usize, col: usize) {
    while row > 0 {
        if grid[row - 1][col] == b'O' || grid[row - 1][col] == b'#' {
            break;
        }

        // Swap grid[row-1][col] and grid[row][col]
        let temp = grid[row - 1][col];
        grid[row - 1][col] = grid[row][col];
        grid[row][col] = temp;

        // Decrement row to move up
        row -= 1;
    }
}

fn move_down(grid: &mut Vec<Vec<u8>>, mut row: usize, col: usize) {
    while row < grid.len() - 1 {
        if grid[row + 1][col] == b'O' || grid[row + 1][col] == b'#' {
            break;
        }

        // Swap grid[row-1][col] and grid[row][col]
        let temp = grid[row + 1][col];
        grid[row + 1][col] = grid[row][col];
        grid[row][col] = temp;

        // Decrement row to move up
        row += 1;
    }
}

fn move_left(grid: &mut Vec<Vec<u8>>, row: usize, mut col: usize) {
    while col > 0 {
        if grid[row][col - 1] == b'O' || grid[row][col - 1] == b'#' {
            break;
        }

        // Swap grid[row-1][col] and grid[row][col]
        let temp = grid[row][col - 1];
        grid[row][col - 1] = grid[row][col];
        grid[row][col] = temp;

        // Decrement row to move up
        col -= 1;
    }
}

fn move_right(grid: &mut Vec<Vec<u8>>, row: usize, mut col: usize) {
    while col < grid[row].len() - 1 {
        if grid[row][col + 1] == b'O' || grid[row][col + 1] == b'#' {
            break;
        }

        // Swap grid[row-1][col] and grid[row][col]
        let temp = grid[row][col + 1];
        grid[row][col + 1] = grid[row][col];
        grid[row][col] = temp;

        // Decrement row to move up
        col += 1;
    }
}

fn format_grid(grid: &Vec<Vec<u8>>) -> String {
    let mut res = String::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            res.push(grid[i][j] as char);
        }
        res.push('\n');
    }
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'O' {
                move_up(&mut grid, i, j);
            }
        }
    }

    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'O' {
                res += grid.len() - i;
            }
        }
    }

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let mut seen = std::collections::HashMap::new();
    let mut loops = 0;
    loop {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == b'O' {
                    move_up(&mut grid, i, j);
                }
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == b'O' {
                    move_left(&mut grid, i, j);
                }
            }
        }
        // reverse from grid.len() -1 to 0
        for i in (0..grid.len()).rev() {
            for j in 0..grid[i].len() {
                if grid[i][j] == b'O' {
                    move_down(&mut grid, i, j);
                }
            }
        }
        for i in 0..grid.len() {
            // reverse from grid[i].len() -1 to 0
            for j in (0..grid[i].len()).rev() {
                if grid[i][j] == b'O' {
                    move_right(&mut grid, i, j);
                }
            }
        }

        if let Some(seen_at) = seen.insert(format_grid(&grid), loops) {
            if (1000000000 - loops) % (loops - seen_at) == 1 {
                break;
            }
        }
        loops += 1;
    }
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'O' {
                res += grid.len() - i;
            }
        }
    }

    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
