use std::collections::HashSet;

advent_of_code::solution!(10);
#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_loop(grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let (start_x, start_y) = grid
        .iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .enumerate()
                .find_map(|(y, &c)| if c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    // figure out start of loop
    let mut x = start_x;
    let mut y = start_y;
    let mut dir = Direction::Up;
    if start_x > 0 && "|7F".contains(grid[start_x - 1][start_y]) {
        x -= 1;
    } else if start_x < grid.len() - 1 && "|LJ".contains(grid[start_x + 1][start_y]) {
        x += 1;
        dir = Direction::Down;
    } else if start_y > 0 && "-LF".contains(grid[start_x][start_y - 1]) {
        y -= 1;
        dir = Direction::Left;
    } else if start_y < grid.len() - 1 && "-J7".contains(grid[start_x][start_y + 1]) {
        y += 1;
        dir = Direction::Right;
    }

    let mut visited = HashSet::new();
    visited.insert((start_x, start_y));
    while !(x == start_x && y == start_y) {
        let cell = grid[x][y];
        visited.insert((x, y));
        match cell {
            '-' => {
                if dir == Direction::Right {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
            '|' => {
                if dir == Direction::Down {
                    x += 1;
                } else {
                    x -= 1;
                }
            }
            '7' => {
                if dir == Direction::Right {
                    x += 1;
                    dir = Direction::Down;
                } else {
                    y -= 1;
                    dir = Direction::Left;
                }
            }
            'F' => {
                if dir == Direction::Left {
                    x += 1;
                    dir = Direction::Down;
                } else {
                    y += 1;
                    dir = Direction::Right;
                }
            }
            'L' => {
                if dir == Direction::Down {
                    y += 1;
                    dir = Direction::Right;
                } else {
                    x -= 1;
                    dir = Direction::Up;
                }
            }
            'J' => {
                if dir == Direction::Down {
                    y -= 1;
                    dir = Direction::Left;
                } else {
                    x -= 1;
                    dir = Direction::Up;
                }
            }
            _ => panic!("Unknown cell: {}", cell),
        }
    }
    visited
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let loop_cells = find_loop(&grid);
    Some(loop_cells.len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let visited = find_loop(&grid);
    let mut container = Vec::new();
    for i in 0..grid.len() {
        let mut within = false;
        for j in 0..grid[i].len() {
            if visited.contains(&(i, j)) {
                let cell = grid[i][j];
                if cell == '|' || cell == 'J' || cell == 'L' || cell == 'S' {
                    within = !within;
                }
            } else if within {
                container.push((i, j));
            }
        }
    }
    Some(container.len() as u32)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
