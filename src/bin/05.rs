advent_of_code::solution!(5);

fn get_seeds(input: &str) -> Vec<&str> {
    input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
}

fn get_maps(input: &str) -> Vec<Vec<(u64, u64, u64)>> {
    input
        .split("\n\n")
        .skip(1)
        .map(|chunk| {
            chunk
                .split(":\n")
                .nth(1)
                .unwrap()
                .lines()
                .map(|line| {
                    let mut iter = line.split_whitespace();
                    let dest = iter.next().unwrap().parse::<u64>().unwrap();
                    let source = iter.next().unwrap().parse::<u64>().unwrap();
                    let offset = iter.next().unwrap().parse::<u64>().unwrap();
                    (dest, source, offset)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let seeds = get_seeds(input);
    let maps = get_maps(input);

    let mut res = std::u64::MAX;
    for seed in seeds {
        let mut seed = seed.parse::<u64>().unwrap();
        for layer in &maps {
            for (dest, source, offset) in layer {
                if seed >= *source && seed <= source + offset {
                    seed = dest + (seed - source);
                    break;
                }
            }
        }
        res = res.min(seed);
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let seeds = get_seeds(input);
    let maps = get_maps(input);

    let mut ranges = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i].parse::<u64>().unwrap();
        let end = seeds[i + 1].parse::<u64>().unwrap() + start;
        ranges.push((start, end));
    }

    for layer in &maps {
        let mut new_ranges = Vec::new();
        while !ranges.is_empty() {
            let (start, end) = ranges.pop().unwrap();
            let mut found = false;
            for (dest, source, offset) in layer {
                let overlap_start = start.max(*source);
                let overlap_end = end.min(source + offset);

                if overlap_start < overlap_end {
                    new_ranges.push((
                        dest + (overlap_start - source),
                        dest + (overlap_end - source),
                    ));
                    if overlap_start > start {
                        ranges.push((start, overlap_start));
                    }
                    if overlap_end < end {
                        ranges.push((overlap_end, end));
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                new_ranges.push((start, end));
            }
        }
        ranges = new_ranges;
    }
    Some(ranges.iter().min().unwrap().0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
