use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(23);

fn dfs(grid: &mut Vec<Vec<u8>>, x: isize, y: isize, res: &mut usize, dir: (isize, isize)) {
    // check if x and y are in bounds
    if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
        return;
    }

    let x_index = x as usize;
    let y_index = y as usize;
    match dir {
        (-1, 0) => {
            // if current cell is v return
            if grid[x_index][y_index] != b'^' && grid[x_index][y_index] != b'.' {
                return;
            }
        }
        (1, 0) => {
            if grid[x_index][y_index] != b'v' && grid[x_index][y_index] != b'.' {
                return;
            }
        }
        (0, -1) => {
            if grid[x_index][y_index] != b'<' && grid[x_index][y_index] != b'.' {
                return;
            }
        }
        (0, 1) => {
            if grid[x_index][y_index] != b'>' && grid[x_index][y_index] != b'.' {
                return;
            }
        }
        _ => unreachable!(),
    }

    // if we are on the last row, we have reached the end
    // print the grid and return
    if x_index == grid.len() - 1 {
        // count number of O's and print grid
        let count = grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c == b'O')
            .count();
        if count > *res {
            *res = count;
        }
    }

    // mark current cell as water
    let original = grid[x_index][y_index];
    grid[x_index][y_index] = b'O';

    for dir in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        dfs(grid, x + dir.0, y + dir.1, res, *dir);
    }

    // mark current cell as original
    grid[x_index][y_index] = original;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    // find the starting point, which is the first . in the first row
    let start = (0, grid[0].iter().position(|&c| c == b'.').unwrap() as isize);

    let mut res = 0;
    // call dfs on the starting point
    dfs(&mut grid, start.0, start.1, &mut res, (1, 0));
    Some(res as u32)
}

fn dfs2(
    graph: &HashMap<(usize, usize), Vec<(usize, usize, usize)>>,
    seen: &mut Vec<Vec<bool>>,
    (r, c): (usize, usize),
) -> Option<usize> {
    if r == seen.len() - 1 {
        return Some(0);
    }
    let mut max_dist = None;
    for &(rr, cc, d) in &graph[&(r, c)] {
        if !seen[rr][cc] {
            seen[rr][cc] = true;
            if let Some(dist) = dfs2(graph, seen, (rr, cc)) {
                max_dist = Some(max_dist.unwrap_or(0).max(d + dist))
            }
            seen[rr][cc] = false;
        }
    }
    max_dist
}

const NEIGHBORS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut graph = HashMap::<_, Vec<_>>::new();
    for (r, c) in (0..grid.len()).cartesian_product(0..grid[0].len()) {
        let neighbors = match grid[r][c] {
            b'#' => continue,
            _ => NEIGHBORS,
        };
        let e = graph.entry((r, c)).or_default();
        for (dr, dc) in neighbors {
            let rr = (r as isize + dr) as usize;
            let cc = (c as isize + dc) as usize;
            if grid
                .get(rr)
                .and_then(|row| row.get(cc))
                .is_some_and(|&t| t != b'#')
            {
                e.push((rr, cc, 1));
            }
        }
    }

    let corridors = graph
        .iter()
        .filter(|(_, n)| n.len() == 2)
        .map(|(&node, _)| node)
        .collect::<Vec<_>>();
    for (r, c) in corridors {
        let neighbors = graph.remove(&(r, c)).unwrap();
        let (r1, c1, d1) = neighbors[0];
        let (r2, c2, d2) = neighbors[1];
        let n1 = graph.get_mut(&(r1, c1)).unwrap();
        if let Some(i) = n1.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n1[i] = (r2, c2, d1 + d2);
        }
        let n2 = graph.get_mut(&(r2, c2)).unwrap();
        if let Some(i) = n2.iter().position(|&(rr, cc, _)| (rr, cc) == (r, c)) {
            n2[i] = (r1, c1, d1 + d2);
        }
    }

    let res = dfs2(
        &graph,
        &mut vec![vec![false; grid[0].len()]; grid.len()],
        (0, 1),
    )
    .unwrap();

    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
