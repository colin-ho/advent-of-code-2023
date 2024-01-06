use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

advent_of_code::solution!(16);

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn step(row: usize, col: usize, dir: Direction) -> Option<(usize, usize, Direction)> {
    match dir {
        Direction::Up => {
            if row > 0 {
                Some((row - 1, col, dir))
            } else {
                None
            }
        }
        Direction::Down => Some((row + 1, col, dir)),
        Direction::Left => {
            if col > 0 {
                Some((row, col - 1, dir))
            } else {
                None
            }
        }
        Direction::Right => Some((row, col + 1, dir)),
    }
}

fn travel(
    grid: &Vec<Vec<u8>>,
    initial_state: (usize, usize, Direction),
    seen: &mut HashSet<(usize, usize, Direction)>,
) {
    let mut stack = vec![initial_state];

    while let Some((row, col, dir)) = stack.pop() {
        if row >= grid.len() || col >= grid[row].len() || seen.contains(&(row, col, dir.clone())) {
            continue;
        }
        seen.insert((row, col, dir.clone()));

        let cell = grid[row][col];
        let mut do_step = |dir: Direction| {
            if let Some(step) = step(row, col, dir) {
                stack.push(step);
            }
        };

        match (cell, dir.clone()) {
            (b'|', Direction::Left | Direction::Right) => {
                do_step(Direction::Up);
                do_step(Direction::Down);
            }
            (b'-', Direction::Up | Direction::Down) => {
                do_step(Direction::Left);
                do_step(Direction::Right);
            }
            (b'\\', _) => match dir {
                Direction::Up => do_step(Direction::Left),
                Direction::Down => do_step(Direction::Right),
                Direction::Left => do_step(Direction::Up),
                Direction::Right => do_step(Direction::Down),
            },
            (b'/', _) => match dir {
                Direction::Up => do_step(Direction::Right),
                Direction::Down => do_step(Direction::Left),
                Direction::Left => do_step(Direction::Down),
                Direction::Right => do_step(Direction::Up),
            },
            _ => {
                do_step(dir);
            }
        }
    }
}

fn count_unique(seen: &HashSet<(usize, usize, Direction)>) -> usize {
    seen.iter()
        .map(|(row, col, _)| (row, col))
        .collect::<HashSet<_>>()
        .len()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut seen = HashSet::new();
    travel(&grid, (0, 0, Direction::Right), &mut seen);
    // count same row and col in seen
    Some(count_unique(&seen) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let res = Arc::new(Mutex::new(0));

    (0..grid.len()).into_par_iter().for_each(|i| {
        let mut seen = HashSet::new();
        let mut local_res = 0;

        travel(&grid, (i, 0, Direction::Right), &mut seen);
        local_res = local_res.max(count_unique(&seen));
        seen.clear();
        travel(&grid, (i, grid[i].len() - 1, Direction::Left), &mut seen);
        local_res = local_res.max(count_unique(&seen));

        let mut res = res.lock().unwrap();
        *res = res.max(local_res);
    });

    (0..grid[0].len()).into_par_iter().for_each(|i| {
        let mut seen = HashSet::new();
        let mut local_res = 0;

        travel(&grid, (0, i, Direction::Down), &mut seen);
        local_res = local_res.max(count_unique(&seen));
        seen.clear();
        travel(&grid, (grid.len() - 1, i, Direction::Up), &mut seen);
        local_res = local_res.max(count_unique(&seen));

        let mut res = res.lock().unwrap();
        *res = res.max(local_res);
    });

    let res = res.lock().unwrap();
    Some(*res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
