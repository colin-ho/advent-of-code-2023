advent_of_code::solution!(25);
use std::collections::HashMap;

use petgraph::{graph::UnGraph, Graph, Undirected};
use rustworkx_core::connectivity::stoer_wagner_min_cut;

fn parse(input: &str) -> Graph<&str, (), Undirected> {
    let mut nodes = HashMap::new();
    let mut graph = UnGraph::new_undirected();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let node = parts.next().unwrap();
        let neighbors = parts.next().unwrap().split_whitespace();

        let node = *nodes.entry(node).or_insert_with(|| graph.add_node(node));
        for neighbor in neighbors {
            let neighbor = *nodes
                .entry(neighbor)
                .or_insert_with(|| graph.add_node(neighbor));
            graph.add_edge(node, neighbor, ());
        }
    }
    graph
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = parse(input);

    let total = graph.node_count();
    let (min_cut_weight, min_cut_set) = stoer_wagner_min_cut(&graph, |_| Ok::<i32, ()>(1))
        .unwrap()
        .unwrap();

    assert_eq!(min_cut_weight, 3);
    ((total - min_cut_set.len()) * min_cut_set.len())
        .try_into()
        .ok()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
