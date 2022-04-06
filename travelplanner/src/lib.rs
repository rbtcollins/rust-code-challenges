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
    fn queue_edges(
        &self,
        pending_edges: &mut BinaryHeap<Reverse<(Cost, Node, Node)>>,
        node: Node,
        cost: Cost,
    ) {
        for (next, next_cost) in self
            .edges
            .get(&node)
            .expect(&format!("invalid graph missing node {node}"))
        {
            pending_edges.push(Reverse((*next_cost + cost, node, *next)));
        }
    }

    // Find a path in the graph
    pub fn ospf(&self, from: Node, to: Node) -> Option<Vec<Node>> {
        // Corner cases
        if !self.nodes.contains(&from) || !self.nodes.contains(&to) {
            return None;
        }
        if from == to {
            return Some(vec![from]);
        }
        // The big show: two working structures: the total cost to get to nodes
        // we have examined, and a queue of edges sorted by the total cost to
        // get to the destination.
        let mut optimal_costs = HashMap::new();
        let mut pending_edges = BinaryHeap::new();
        optimal_costs.insert(from, (from, 0));
        self.queue_edges(&mut pending_edges, from, 0);

        loop {
            match pending_edges
                .pop()
                .map(|Reverse((candidate_dest_cost, source, dest))| {
                    // look up any existing best cost to get to dest
                    optimal_costs.entry(dest).or_insert_with(|| {
                        self.queue_edges(&mut pending_edges, dest, candidate_dest_cost);
                        (source, candidate_dest_cost)
                    });
                    dest
                }) {
                None => return None,
                Some(e) if e == to => {
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
                _ => {}
            };
        }
    }
}
