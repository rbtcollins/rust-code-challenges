use std::collections::HashMap;
use std::fs::File;
use std::io::Result;

use clap::Parser;
use rand::Rng;

use travelplanner::*;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    graph_path: std::path::PathBuf,
    size: usize,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut graph = Graph::default();
    let mut inverse_edges = HashMap::<Node, Vec<Node>>::new();
    let mut rng = rand::thread_rng();
    // add nodes
    for i in 0..args.size {
        graph.nodes.insert(i);
    }
    // add connections. We add 6x the node size. It takes two connections to go
    // from A to B to A, so this allows about 3 paths into or out of each node.
    // nodes are capped at 4 in and 4 out - forcing more steps through the graph
    // rather than small world network effects.
    for _ in 0..args.size {
        let mut from = rng.gen_range(0..args.size);
        let mut to = rng.gen_range(0..args.size);
        let cost = rng.gen_range(0..20);
        // linear probe for the next node with an available spot.
        loop {
            if graph
                .edges
                .get(&from)
                .map(|v| v.iter().filter(|e| e.0 == from).count())
                .unwrap_or_default()
                < 4
            {
                break;
            }
            from = (from + 1) % args.size;
        }

        loop {
            if inverse_edges
                .get(&to)
                .map(|v| v.iter().filter(|e| **e == to).count())
                .unwrap_or_default()
                < 4
            {
                break;
            }
            to = (to + 1) % args.size;
        }

        graph
            .edges
            .entry(from)
            .or_insert(Vec::new())
            .push((to, cost));
        inverse_edges.entry(to).or_insert(Vec::new()).push(from);
    }
    let output = File::create(args.graph_path)?;
    serde_json::to_writer(output, &graph)?;

    println!("Created graph");
    Ok(())
}
