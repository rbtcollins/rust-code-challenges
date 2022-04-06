use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use serde::{Deserialize, Serialize};

pub type Node = usize;

pub type Cost = usize;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Graph {
    pub edges: HashMap<Node, Vec<(Node, Cost)>>,
    pub nodes: HashSet<Node>,
}

impl Graph {
    fn queue_edges(&self, pending_edges: &mut BinaryHeap<Reverse<(Cost, Node, Node)>>, node: Node) {
        for (next, next_cost) in self
            .edges
            .get(&node)
            .expect(&format!("invalid graph missing node {node}"))
        {
            pending_edges.push(Reverse((*next_cost, node, *next)));
        }
    }
    pub fn ospf(&self, from: Node, to: Node) -> Option<Vec<Node>> {
        if !self.nodes.contains(&from) || !self.nodes.contains(&to) {
            return None;
        }
        if from == to {
            return Some(vec![from]);
        }
        let mut optimal_costs = HashMap::new();
        let mut pending_edges = BinaryHeap::new();
        optimal_costs.insert(from, (from, 0));
        self.queue_edges(&mut pending_edges, from);

        loop {
            match pending_edges
                .pop()
                .map(|Reverse((hop_cost, source, dest))| {
                    // look up the existing best cost to get to source
                    let source_cost = optimal_costs.get(&source).unwrap().1;
                    let candidate_dest_cost = hop_cost + source_cost;
                    // look up any existing best cost to get to dest
                    optimal_costs
                        .entry(dest)
                        .and_modify(|(optimal_source, optimal_cost)| {
                            if *optimal_cost > candidate_dest_cost {
                                *optimal_source = source;
                                *optimal_cost = candidate_dest_cost;
                            }
                        })
                        .or_insert_with(|| {
                            self.queue_edges(&mut pending_edges, dest);
                            (source, candidate_dest_cost)
                        });
                    dest
                }) {
                None => return None,
                Some(e) => {
                    if e == to {
                        let mut answer = vec![];
                        let mut current_node = &to;
                        answer.push(*current_node);
                        while *current_node != from {
                            if let Some((predecessor, _cost)) = optimal_costs.get(&current_node) {
                                current_node = predecessor;
                                answer.push(*current_node);
                            } else {
                                unreachable!("logic error: we ended up with an impossible state");
                            }
                        }
                        answer.reverse();
                        return Some(answer);
                    }
                }
            };
        }
    }
}
