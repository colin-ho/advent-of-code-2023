use std::collections::HashMap;

advent_of_code::solution!(19);

struct Rule {
    category: char,
    comparator: char,
    rating: u64,
    destination: String,
}

fn create_workflow_map(rules: &str) -> HashMap<String, Vec<Rule>> {
    let workflow_map: HashMap<String, Vec<Rule>> = rules
        .lines()
        .map(|line| {
            let mut parts = line.split("{");
            let name = parts.next().unwrap();

            let rules_string = parts.next().unwrap().trim_end_matches('}');
            let rules_vec = rules_string.split(',').map(|rule| {
                let rule_parts: Vec<&str> = rule.split(':').collect();
                if rule_parts.len() == 1 {
                    Rule {
                        category: ' ',
                        comparator: ' ',
                        rating: 0,
                        destination: rule_parts[0].to_string(),
                    }
                } else {
                    let category = rule_parts[0].chars().next().unwrap();
                    let comparator = rule_parts[0].chars().nth(1).unwrap();
                    let rating = rule_parts[0][2..].parse::<u64>().unwrap();
                    let destination = rule_parts[1].to_string();
                    Rule {
                        category,
                        comparator,
                        rating,
                        destination,
                    }
                }
            });
            (name.to_string(), rules_vec.collect())
        })
        .collect();
    workflow_map
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let workflow_map: HashMap<String, Vec<Rule>> = create_workflow_map(rules);

    parts.lines().fold(None, |acc, line| {
        let line = line.trim_matches(|c| c == '{' || c == '}');
        let part: HashMap<char, u64> = line
            .split(',')
            .map(|category| {
                let category_split: Vec<&str> = category.split('=').collect();
                (
                    category_split[0].chars().next().unwrap(),
                    category_split[1].parse::<u64>().unwrap(),
                )
            })
            .collect();

        let mut accepted = false;
        let mut workflow = "in";
        'outer: loop {
            let rules = workflow_map.get(workflow).unwrap();
            for rule in rules {
                if rule.category == ' ' {
                    if rule.destination == "A" {
                        accepted = true;
                        break 'outer;
                    } else if rule.destination == "R" {
                        accepted = false;
                        break 'outer;
                    } else {
                        workflow = &rule.destination;
                        continue 'outer;
                    }
                }
                let value = part.get(&rule.category).unwrap();
                if (rule.comparator == '<' && rule.rating > *value)
                    || (rule.comparator == '>' && rule.rating < *value)
                {
                    if rule.destination == "A" {
                        accepted = true;
                        break 'outer;
                    } else if rule.destination == "R" {
                        accepted = false;
                        break 'outer;
                    } else {
                        workflow = &rule.destination;
                        continue 'outer;
                    }
                }
            }
        }
        if accepted {
            // add sum of part values to acc
            let sum = part.values().sum::<u64>();
            if let Some(acc) = acc {
                Some(acc + sum)
            } else {
                Some(sum)
            }
        } else {
            acc
        }
    })
}

fn count(
    mut ranges: HashMap<char, (u64, u64)>,
    name: String,
    workflow_map: &HashMap<String, Vec<Rule>>,
) -> u64 {
    if name == 'R'.to_string() {
        return 0;
    }
    if name == 'A'.to_string() {
        let mut product = 1;
        for (_, range) in ranges {
            product *= range.1 - range.0 + 1;
        }
        return product;
    }
    let rules = workflow_map.get(&name).unwrap();
    let mut total = 0;
    for rule in rules {
        if rule.category == ' ' {
            total += count(ranges.clone(), rule.destination.clone(), workflow_map);
        } else {
            let (lo, hi) = ranges.get(&rule.category).unwrap();
            let t = if rule.comparator == '<' {
                let upper = rule.rating - 1;
                (*lo, upper.min(*hi))
            } else {
                let lower = rule.rating + 1;
                (lower.max(*lo), *hi)
            };
            let f = if rule.comparator == '<' {
                let lower = rule.rating;
                (lower.max(*lo), *hi)
            } else {
                let upper = rule.rating;
                (*lo, upper.min(*hi))
            };

            if t.0 <= t.1 {
                let mut new_ranges = ranges.clone();
                new_ranges.insert(rule.category, t);
                total += count(new_ranges, rule.destination.clone(), workflow_map);
            }

            if f.0 <= f.1 {
                ranges = ranges.clone();
                ranges.insert(rule.category, f);
            } else {
                break;
            }
        }
    }
    total
}

pub fn part_two(input: &str) -> Option<u64> {
    let rules = input.split("\n\n").nth(0).unwrap();
    let workflow_map: HashMap<String, Vec<Rule>> = create_workflow_map(rules);

    let mut ranges: HashMap<char, (u64, u64)> = HashMap::new();
    ranges.insert('x', (1, 4000));
    ranges.insert('m', (1, 4000));
    ranges.insert('a', (1, 4000));
    ranges.insert('s', (1, 4000));

    Some(count(ranges, "in".to_string(), &workflow_map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
