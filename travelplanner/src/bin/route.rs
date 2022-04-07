use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Result},
    time::Instant,
};

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
    let mut input = File::open(&args.graph_path)?;
    let mut buf = vec![];
    input.read_to_end(&mut buf)?;
    let graph: Graph = bincode::deserialize(&buf[..])
        .map_err(|e| Error::new(ErrorKind::Other, format!("{e:?}")))?;
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
