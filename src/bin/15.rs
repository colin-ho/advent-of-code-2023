advent_of_code::solution!(15);

fn hash(s: &str) -> usize {
    let hash = s.chars().fold(0, |acc, c| {
        // increase acc by ascii value of c
        let mut acc = acc;
        acc += c as u64;
        // multiply acc by 17
        acc *= 17;
        // set acc to be remained of acc and 256
        acc %= 256;
        acc
    });
    hash as usize
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split(',')
        .fold(Some(0), |acc, x| Some(acc? + hash(x) as u32))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lens_boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    input.split(',').for_each(|x| {
        if x.contains('=') {
            let (label, focal_length) = x.split_once('=').unwrap();
            let hash = hash(label);

            let mut found = false;
            if let Some(lens_box) = lens_boxes.get_mut(hash) {
                for lens in lens_box {
                    if lens.label == label {
                        lens.focal_length = focal_length.parse::<u32>().unwrap();
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                lens_boxes[hash].push(Lens {
                    label: label.to_string(),
                    focal_length: focal_length.parse::<u32>().unwrap(),
                });
            }
        } else {
            let label = x.trim_end_matches('-');
            let hash = hash(label);

            // remove label from lens_boxes in lens_boxes[hash] if it exists
            if let Some(lens_box) = lens_boxes.get_mut(hash) {
                lens_box.retain(|lens| lens.label != label);
            }
        }
    });

    let mut res = 0;
    for (i, lenses) in lens_boxes.iter().enumerate() {
        if !lenses.is_empty() {
            for (j, lens) in lenses.iter().enumerate() {
                res += (i as u32 + 1) * (j as u32 + 1) * lens.focal_length;
            }
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
