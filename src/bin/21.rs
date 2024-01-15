use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(21);

fn bfs(grid: &[&[u8]], start: (isize, isize), steps: usize) -> usize {
    let mut pos = HashSet::from_iter([start]);
    let mut new_pos = HashSet::new();
    for _ in 0..steps {
        new_pos.clear();
        for &(r, c) in &pos {
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (rr, cc) = (r + dr, c + dc);
                let tile = grid[rr as usize % grid.len()][cc as usize % grid.len()];
                if tile != b'#' {
                    new_pos.insert((rr, cc));
                }
            }
        }
        (pos, new_pos) = (new_pos, pos);
    }
    pos.len()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let start = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(r, c)| grid[r][c] == b'S')
        .map(|(r, c)| (r as isize, c as isize))
        .unwrap();

    Some(bfs(&grid, start, 64) as u32)
}

fn fill(grid: &Vec<Vec<char>>, sr: usize, sc: usize, ss: usize) -> usize {
    let mut ans = HashSet::new();
    let mut seen = HashSet::new();
    seen.insert((sr, sc));
    let mut q = VecDeque::new();
    q.push_back((sr, sc, ss));

    while let Some((r, c, s)) = q.pop_front() {
        if s % 2 == 0 {
            ans.insert((r, c));
        }
        if s == 0 {
            continue;
        }

        for &(nr, nc) in &[
            (r.wrapping_add(1), c),
            (r.wrapping_sub(1), c),
            (r, c.wrapping_add(1)),
            (r, c.wrapping_sub(1)),
        ] {
            if nr >= grid.len()
                || nc >= grid[0].len()
                || grid[nr][nc] == '#'
                || seen.contains(&(nr, nc))
            {
                continue;
            }
            seen.insert((nr, nc));
            q.push_back((nr, nc, s - 1));
        }
    }

    ans.len()
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (sr, sc) = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, &ch)| if ch == 'S' { Some((r, c)) } else { None })
        })
        .unwrap();

    let size = grid.len();
    let steps: usize = 26501365;

    let grid_width = steps / size - 1;

    let odd = (grid_width / 2 * 2 + 1).pow(2);
    let even = ((grid_width + 1) / 2 * 2).pow(2);

    let odd_points = fill(&grid, sr, sc, size * 2 + 1);
    let even_points = fill(&grid, sr, sc, size * 2);

    let corner_t = fill(&grid, size - 1, sc, size - 1);
    let corner_r = fill(&grid, sr, 0, size - 1);
    let corner_b = fill(&grid, 0, sc, size - 1);
    let corner_l = fill(&grid, sr, size - 1, size - 1);

    let small_tr = fill(&grid, size - 1, 0, size / 2 - 1);
    let small_tl = fill(&grid, size - 1, size - 1, size / 2 - 1);
    let small_br = fill(&grid, 0, 0, size / 2 - 1);
    let small_bl = fill(&grid, 0, size - 1, size / 2 - 1);

    let large_tr = fill(&grid, size - 1, 0, size * 3 / 2 - 1);
    let large_tl = fill(&grid, size - 1, size - 1, size * 3 / 2 - 1);
    let large_br = fill(&grid, 0, 0, size * 3 / 2 - 1);
    let large_bl = fill(&grid, 0, size - 1, size * 3 / 2 - 1);

    let result = odd * odd_points
        + even * even_points
        + corner_t
        + corner_r
        + corner_b
        + corner_l
        + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
        + grid_width * (large_tr + large_tl + large_br + large_bl);

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2691));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(470149860542205));
    }
}
