use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::token_parser::token_parser_benches
}
