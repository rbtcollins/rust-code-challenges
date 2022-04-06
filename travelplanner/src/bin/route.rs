use std::{fs::File, io::Result, time::Instant};

use clap::Parser;

use travelplanner::*;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    graph_path: std::path::PathBuf,
    from: Node,
    to: Node,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let input = File::open(&args.graph_path)?;
    let input = std::io::BufReader::with_capacity(1024 * 1024, input);
    let graph: Graph = serde_json::from_reader(input)?;
    let start = Instant::now();
    let route = graph.ospf(args.from, args.to);
    let duration = Instant::now() - start;
    println!("Route took {duration:?} to find {route:?}");
    Ok(())
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn
}
