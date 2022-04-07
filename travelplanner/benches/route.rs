use std::{fs::File, io::Read};

use criterion::{/* black_box, */ criterion_group, criterion_main, Criterion};

use travelplanner::Graph;

fn criterion_benchmark(c: &mut Criterion) {
    let mut input = File::open("graph.bincode").unwrap();
    let mut buf = vec![];
    input.read_to_end(&mut buf).unwrap();
    let graph: Graph = bincode::deserialize(&buf[..]).unwrap();
    c.bench_function("route 1 32946", |b| b.iter(|| graph.ospf(1, 32946)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
