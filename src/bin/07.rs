advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| {
            let (card, bid) = line.split_once(' ').unwrap();
            let rank = card
                .as_bytes()
                .iter()
                .map(|&b| match b {
                    b'A' => 14,
                    b'K' => 13,
                    b'Q' => 12,
                    b'J' => 11,
                    b'T' => 10,
                    _ => b.saturating_sub(b'0') as u64,
                })
                .collect::<Vec<_>>();

            let mut freq = [0; 15];
            rank.iter().for_each(|&r| freq[r as usize] += 1);
            freq.sort_unstable();
            freq.reverse();

            let mut sort_key: u64 = 0;
            for f in freq.iter().take(5) {
                sort_key = (sort_key << 4) | f;
            }
            for r in rank.iter().take(5) {
                sort_key = (sort_key << 4) | r;
            }

            (sort_key, bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * bid)
        .sum::<u32>()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| {
            let (card, bid) = line.split_once(' ').unwrap();
            let rank = card
                .as_bytes()
                .iter()
                .map(|&b| match b {
                    b'A' => 14,
                    b'K' => 13,
                    b'Q' => 12,
                    b'J' => 1,
                    b'T' => 10,
                    _ => b.saturating_sub(b'0') as u64,
                })
                .collect::<Vec<_>>();

            let mut freq = [0; 15];
            rank.iter().for_each(|&r| freq[r as usize] += 1);
            let jokers = freq[1];
            freq[1] = 0;
            freq.sort_unstable();
            freq.reverse();
            freq[0] += jokers;

            let mut sort_key: u64 = 0;
            for f in freq.iter().take(5) {
                sort_key = (sort_key << 4) | f;
            }
            for r in rank.iter().take(5) {
                sort_key = (sort_key << 4) | r;
            }

            (sort_key, bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * bid)
        .sum::<u32>()
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
