use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

struct Module {
    module_type: char,
    destinations: Vec<String>,
    is_on: bool,
    last_inputs: HashMap<String, String>,
}

fn parse_modules(input: &str) -> (HashMap<String, Module>, String) {
    let mut conjunction_modules = Vec::new();
    let mut rx_feeder: String = "".to_string();
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(|line| {
            let (module, destinations) = line.split_once(" -> ").unwrap();
            let module_type = module.chars().next().unwrap();
            let module_name: String = module.chars().skip(1).collect();
            if module_type == '&' {
                conjunction_modules.push(module_name.clone());
            }
            let destinations: Vec<String> =
                destinations.split(", ").map(|s| s.to_string()).collect();
            if destinations.contains(&"rx".to_string()) {
                rx_feeder = module_name.clone();
            }
            (
                module_name,
                Module {
                    module_type,
                    destinations,
                    is_on: false,
                    last_inputs: HashMap::new(),
                },
            )
        })
        .collect();

    let mut destinations_for_conjunction_modules: HashMap<String, Vec<String>> = HashMap::new();
    for module_name in conjunction_modules {
        for (n, module) in &mut modules {
            if module.destinations.contains(&module_name) {
                destinations_for_conjunction_modules
                    .entry(module_name.clone())
                    .or_insert(Vec::new())
                    .push(n.clone());
            }
        }
    }

    for (name, module) in &mut modules {
        if destinations_for_conjunction_modules.contains_key(name) {
            for destination in destinations_for_conjunction_modules.get(name).unwrap() {
                module
                    .last_inputs
                    .insert(destination.clone(), "low".to_string());
            }
        }
    }

    (modules, rx_feeder)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut modules = parse_modules(input).0;

    let mut num_low_pulses = 0;
    let mut num_high_pulses = 0;
    for _ in 0..1000 {
        let mut q = VecDeque::new();
        q.push_back((
            "roadcaster".to_string(),
            "low".to_string(),
            "button".to_string(),
        ));

        while let Some((module_name, signal, from)) = q.pop_front() {
            if signal == "low" {
                num_low_pulses += 1;
            } else {
                num_high_pulses += 1;
            }
            if let Some(module) = modules.get_mut(&module_name) {
                match module.module_type {
                    'b' => {
                        for destination in &module.destinations {
                            q.push_back((destination.clone(), signal.clone(), module_name.clone()));
                        }
                    }
                    '%' => {
                        if signal == "low" {
                            let new_signal = if module.is_on { "low" } else { "high" };
                            for destination in &module.destinations {
                                q.push_back((
                                    destination.clone(),
                                    new_signal.to_string(),
                                    module_name.clone(),
                                ));
                            }
                            module.is_on = !module.is_on;
                        }
                    }
                    '&' => {
                        module
                            .last_inputs
                            .insert(from.to_string(), signal.to_string());
                        // if all of last inputs are high then send low, else send high
                        let new_signal = if module.last_inputs.values().all(|s| s == "high") {
                            "low"
                        } else {
                            "high"
                        };
                        for destination in &module.destinations {
                            q.push_back((
                                destination.clone(),
                                new_signal.to_string(),
                                module_name.clone(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Some(num_low_pulses * num_high_pulses)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        let temp = m;
        m = n % m;
        n = temp;
    }
    n
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut modules, rx_feeder) = parse_modules(input);

    if rx_feeder == "" {
        return None;
    }

    let mut lengths = HashMap::new();
    let mut visited = HashMap::new();
    for (name, module) in &modules {
        if module.destinations.contains(&rx_feeder) {
            visited.insert(name.clone(), 0);
        }
    }

    let mut res = 0;

    'outer: for i in 1.. {
        let mut q = VecDeque::new();
        q.push_back((
            "roadcaster".to_string(),
            "low".to_string(),
            "button".to_string(),
        ));

        while let Some((module_name, signal, from)) = q.pop_front() {
            if module_name == rx_feeder && signal == "high" {
                // increment visited[from] by 1
                let mut visited_from = visited.get(&from).unwrap().clone();
                visited_from += 1;
                visited.insert(from.clone(), visited_from);

                // if from not in lengths, add
                if !lengths.contains_key(&from) {
                    lengths.insert(from.clone(), i);
                }

                // if all visited values greater than 0
                if visited.values().all(|v| *v > 0) {
                    let mut product = 1;
                    for (_, length) in &lengths {
                        product = product * length / gcd(product, *length);
                    }
                    res = product;
                    break 'outer;
                }
            }
            if let Some(module) = modules.get_mut(&module_name) {
                match module.module_type {
                    'b' => {
                        for destination in &module.destinations {
                            q.push_back((destination.clone(), signal.clone(), module_name.clone()));
                        }
                    }
                    '%' => {
                        if signal == "low" {
                            let new_signal = if module.is_on { "low" } else { "high" };
                            for destination in &module.destinations {
                                q.push_back((
                                    destination.clone(),
                                    new_signal.to_string(),
                                    module_name.clone(),
                                ));
                            }
                            module.is_on = !module.is_on;
                        }
                    }
                    '&' => {
                        module
                            .last_inputs
                            .insert(from.to_string(), signal.to_string());
                        // if all of last inputs are high then send low, else send high
                        let new_signal = if module.last_inputs.values().all(|s| s == "high") {
                            "low"
                        } else {
                            "high"
                        };
                        for destination in &module.destinations {
                            q.push_back((
                                destination.clone(),
                                new_signal.to_string(),
                                module_name.clone(),
                            ));
                        }
                    }
                    _ => {}
                }
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
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
