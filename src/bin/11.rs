advent_of_code::solution!(11);
use itertools::Itertools;

fn find_distances(grid: &Vec<&[u8]>, scale: usize) -> Option<u64> {
    // empty rows are rows with all . get the index of all empty rows
    let empty_rows = (0..grid.len())
        .filter(|&i| grid[i].iter().all(|&cell| cell == b'.'))
        .collect::<Vec<_>>();
    // empty columns are columns with all . get the index of all empty columns

    let empty_cols = (0..grid[0].len())
        .filter(|&j| (0..grid.len()).all(|i| grid[i][j] == b'.'))
        .collect::<Vec<_>>();

    // galaxies are indicated with #, find coordinates of all galaxies
    let galaxies = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(r, c)| grid[r][c] == b'#')
        .collect::<Vec<_>>();

    let mut distances = vec![];

    for (i, &coord1) in galaxies.iter().enumerate() {
        for &coord2 in galaxies.iter().skip(i + 1) {
            // distance between coord 1 and coord 2
            let smaller_row = coord1.0.min(coord2.0);
            let larger_row = coord1.0.max(coord2.0);
            let smaller_col = coord1.1.min(coord2.1);
            let larger_col = coord1.1.max(coord2.1);

            let mut dist = larger_row - smaller_row + larger_col - smaller_col;

            dist += empty_rows
                .iter()
                .filter(|&&r| r < larger_row && r > smaller_row)
                .count()
                * scale;

            dist += empty_cols
                .iter()
                .filter(|&&c| c < larger_col && c > smaller_col)
                .count()
                * scale;

            distances.push(dist);
        }
    }
    // sum of all distances
    let sum = distances.iter().sum::<usize>();
    Some(sum.try_into().unwrap())
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    find_distances(&grid, 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    find_distances(&grid, 999999)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
