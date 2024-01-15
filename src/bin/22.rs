use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(22);

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct Brick {
    start_x: usize,
    start_y: usize,
    start_z: usize,
    end_x: usize,
    end_y: usize,
    end_z: usize,
    id: usize,
}

fn parse(
    input: &str,
) -> (
    usize,
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    // parse input
    let mut counter = 0;
    let mut bricks: Vec<Brick> = input
        .lines()
        .map(|line| {
            let mut parts = line.split("~");
            let mut start = parts.next().unwrap().split(",");
            let mut end = parts.next().unwrap().split(",");
            counter += 1;
            Brick {
                start_x: start.next().unwrap().parse().unwrap(),
                start_y: start.next().unwrap().parse().unwrap(),
                start_z: start.next().unwrap().parse().unwrap(),
                end_x: end.next().unwrap().parse().unwrap(),
                end_y: end.next().unwrap().parse().unwrap(),
                end_z: end.next().unwrap().parse().unwrap(),
                id: counter,
            }
        })
        .collect();

    // create a 3d grid with sizes using the max x, y, z
    let max_x = bricks.iter().map(|b| b.end_x).max().unwrap();
    let max_y = bricks.iter().map(|b| b.end_y).max().unwrap();
    let max_z = bricks.iter().map(|b| b.end_z).max().unwrap();

    let mut grid = vec![vec![vec![0; max_z + 1]; max_y + 1]; max_x + 1];

    // sort bricks by start_z, and keep moving them down until they hit something
    bricks.sort_by(|a, b| a.start_z.cmp(&b.start_z));

    for brick in bricks.iter_mut() {
        let mut start_z = brick.start_z;
        let mut end_z = brick.end_z;
        while start_z - 1 >= 1 {
            let mut can_move = true;
            for x in brick.start_x..=brick.end_x {
                for y in brick.start_y..=brick.end_y {
                    if grid[x][y][start_z - 1] != 0 {
                        can_move = false;
                    }
                }
            }
            if !can_move {
                break;
            }
            start_z -= 1;
            end_z -= 1;
        }

        brick.start_z = start_z;
        brick.end_z = end_z;

        // mark the brick as placed
        for x in brick.start_x..=brick.end_x {
            for y in brick.start_y..=brick.end_y {
                for z in brick.start_z..=brick.end_z {
                    grid[x][y][z] = brick.id;
                }
            }
        }
    }

    let mut bricks_under: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut bricks_over: HashMap<usize, HashSet<usize>> = HashMap::new();
    // for each block find all the blocks directly under it
    for brick in bricks.iter() {
        for x in brick.start_x..=brick.end_x {
            for y in brick.start_y..=brick.end_y {
                let id = grid[x][y][brick.start_z - 1];
                if id != 0 {
                    // if brick.id not in bricks_under[brick.id] create new hashset and add id
                    if !bricks_under.contains_key(&brick.id) {
                        bricks_under.insert(brick.id, HashSet::new());
                    }
                    bricks_under.get_mut(&brick.id).unwrap().insert(id);
                }
                let id = grid[x][y][brick.end_z + 1];
                if id != 0 {
                    if !bricks_over.contains_key(&brick.id) {
                        bricks_over.insert(brick.id, HashSet::new());
                    }
                    bricks_over.get_mut(&brick.id).unwrap().insert(id);
                }
            }
        }
    }

    (counter, bricks_under, bricks_over)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (counter, bricks_under, bricks_over) = parse(input);
    let mut res = 0;
    for brick in 1..=counter {
        let mut stable = true;
        // if brick not in bricks_over then just add 1 to res
        if !bricks_over.contains_key(&brick) {
            res += 1;
            continue;
        }
        for brick_over in bricks_over.get(&brick).unwrap().iter() {
            if bricks_under.get(brick_over).unwrap().len() == 1 {
                stable = false;
                break;
            }
        }
        if stable {
            res += 1;
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (counter, bricks_under, bricks_over) = parse(input);
    let mut res = 0;
    for brick in 1..=counter {
        // for each brick find how many bricks would fall if it was removed
        let mut falling_bricks = HashSet::new();
        let mut to_check = VecDeque::new();
        to_check.push_back(brick);
        while !to_check.is_empty() {
            let id = to_check.pop_front().unwrap();
            // if id not in bricks_over continue
            falling_bricks.insert(id);
            if !bricks_over.contains_key(&id) {
                continue;
            }
            for brick_over in bricks_over.get(&id).unwrap_or(&HashSet::new()) {
                // if all bricks in bricks_under[brick_over] is in falling bricks, add brick_over to falling bricks
                if bricks_under
                    .get(brick_over)
                    .unwrap_or(&HashSet::new())
                    .iter()
                    .all(|b| falling_bricks.contains(b))
                {
                    to_check.push_back(*brick_over);
                }
            }
        }
        falling_bricks.remove(&brick);
        res += falling_bricks.len();
    }

    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
