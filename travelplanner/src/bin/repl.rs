use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};

use clap::Parser;
use repl_rs::{Command, Convert, Parameter, Repl, Result as ReplResult, Value};

use travelplanner::*;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    graph_path: std::path::PathBuf,
}

fn route(args: HashMap<String, Value>, graph: &mut Graph) -> ReplResult<Option<String>> {
    let from: Node = args["from"].convert()?;
    let to: Node = args["to"].convert()?;
    let result = graph.ospf(from, to);
    Ok(Some(format!("route from {from} to {to} is {result:?}")))
}

fn node(args: HashMap<String, Value>, graph: &mut Graph) -> ReplResult<Option<String>> {
    let node: Node = args["node"].convert()?;
    Ok(Some(
        graph
            .edges
            .get(&node)
            .map(|edges| {
                let edges = edges
                    .iter()
                    .map(|(id, cost)| format!("->{id}[cost={cost}]\n"))
                    .collect::<Vec<_>>();
                format!("node {node}:\n {edges:?}")
            })
            .unwrap_or("Unknown node".into()),
    ))
}

fn repl(graph: Graph) -> ReplResult<()> {
    let mut repl = Repl::new(graph)
        .with_name("TravelPlanner")
        .with_version("v0.0.1")
        .with_description("Interactive REPL")
        .add_command(
            Command::new("route", route)
                .with_parameter(Parameter::new("from").set_required(true)?)?
                .with_parameter(Parameter::new("to").set_required(true)?)?
                .with_help("Calculate a route"),
        )
        .add_command(
            Command::new("node", node)
                .with_parameter(Parameter::new("node").set_required(true)?)?
                .with_help("Show a single node"),
        );
    repl.run()
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut input = File::open(&args.graph_path)?;
    let mut buf = vec![];
    input.read_to_end(&mut buf)?;
    let graph: Graph = bincode::deserialize(&buf[..])
        .map_err(|e| Error::new(ErrorKind::Other, format!("{e:?}")))?;

    repl(graph).map_err(|e| Error::new(ErrorKind::Other, format!("{e:?}")))
}
