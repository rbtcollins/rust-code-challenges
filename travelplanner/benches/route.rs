use std::fs::File;

use criterion::{/* black_box, */ criterion_group, criterion_main, Criterion};

use travelplanner::Graph;

fn criterion_benchmark(c: &mut Criterion) {
    let input = File::open("graph.json").unwrap();
    let input = std::io::BufReader::with_capacity(1024 * 1024, input);
    let graph: Graph = serde_json::from_reader(input).unwrap();
    c.bench_function("route 2463 30350", |b| b.iter(|| graph.ospf(2463, 30350)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
