use itertools::Itertools;

advent_of_code::solution!(18);

fn calculate_area(instructions: &Vec<(&str, i64)>) -> i64 {
    let mut position: (i64, i64) = (0, 0);
    let mut area = 0;
    let mut perimiter = 0;

    for (dir, dist) in instructions {
        let prev = position;
        match dir {
            &"U" => {
                position.0 -= *dist;
            }
            &"D" => {
                position.0 += *dist;
            }
            &"L" => {
                position.1 -= *dist;
            }
            &"R" => {
                position.1 += *dist;
            }
            _ => panic!("Unknown direction {}", dir),
        }
        area += (prev.0 * position.1) - (prev.1 * position.0);
        perimiter += dist;
    }
    area.abs() / 2 + perimiter / 2 + 1
}

pub fn part_one(input: &str) -> Option<i64> {
    let instructions = input
        .lines()
        .map(|line| {
            let line = line.split_ascii_whitespace().collect_vec();
            let dir = line[0];
            let dist = line[1].parse::<i64>().unwrap();

            (dir, dist)
        })
        .collect_vec();
    Some(calculate_area(&instructions))
}

pub fn part_two(input: &str) -> Option<i64> {
    let instructions = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("#").collect();
            // distance is slice from 0 to 4, parse as hex
            let dist = i64::from_str_radix(&parts[1][0..5], 16).unwrap();
            let dir = match &parts[1][5..6] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => {
                    // error
                    ""
                }
            };
            (dir, dist)
        })
        .collect_vec();
    Some(calculate_area(&instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
