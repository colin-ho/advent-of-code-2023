use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(17);

#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct State {
    cost: i64,
    position: (usize, usize),
    direction: (isize, isize),
}

fn dijkstra(grid: &Vec<Vec<u8>>, min_step: usize, max_step: usize) -> i64 {
    let mut distances = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        position: (0, 0),
        direction: (0, 0),
    });

    while let Some(State {
        cost,
        position,
        direction,
    }) = queue.pop()
    {
        if position == (grid.len() - 1, grid[0].len() - 1) {
            return -cost;
        }

        if let Some(&d) = distances.get(&(position, direction)) {
            if -cost > d {
                continue;
            }
        }

        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if (*dx, *dy) == (-direction.0, -direction.1)
                || (*dx, *dy) == (direction.0, direction.1)
            {
                continue;
            }
            let mut next_cost = -cost;
            for distance in 1..=max_step {
                let new_x = position.0 as isize + dx * distance as isize;
                let new_y = position.1 as isize + dy * distance as isize;
                if new_x < 0
                    || new_y < 0
                    || new_x >= grid.len() as isize
                    || new_y >= grid[0].len() as isize
                {
                    continue;
                }
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                next_cost += (grid[new_x][new_y] - b'0') as i64;
                if distance < min_step {
                    continue;
                }
                let key = ((new_x, new_y), (*dx, *dy));
                if next_cost < *distances.get(&key).unwrap_or(&i64::MAX) {
                    distances.insert(key, next_cost);
                    queue.push(State {
                        cost: -next_cost,
                        position: (new_x, new_y),
                        direction: (*dx, *dy),
                    });
                }
            }
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<i64> {
    let grid = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Some(dijkstra(&grid, 1, 3))
}

pub fn part_two(input: &str) -> Option<i64> {
    let grid = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Some(dijkstra(&grid, 4, 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
