use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group};
use kusto_language::token_parser::{ParseOptions, parse_tokens};

fn build_long_input() -> String {
    let base = concat!(
        "// comment with punctuation: ()[]{}.,;:+-*/=<>|&^%\n",
        "#directive\n",
        "let t = datatable(x:int, y:long, z:real, s:string, g:guid, b:bool) [",
        "  1, 2, 3.14, \"text\\nline\", guid(00000000-0000-0000-0000-000000000000), true,",
        "  42, 99, 6.02e23, @\"verbatim \"\"string\"\"\", 11111111-2222-3333-4444-555555555555, false",
        "];\n",
        "t\n",
        "| extend ts = datetime(2024-01-02T03:04:05Z), span = time(1.02:03:04)\n",
        "| extend h = h\"hex\", v = h@\"hex verbatim\"\n",
        "| project x, y, z, s, g, b, ts, span\n",
        "| summarize count(), dcount(s) by bin(ts, 1h)\n",
        "| join kind=inner (t | project x, s) on x\n",
        "| union (t | where s contains_cs \"TeSt\")\n",
        "| parse s with * ' name ' *\n",
        "| evaluate bag_unpack(dynamic({\"k\":1,\"v\":[1,2,3]}))\n",
        "| mv-expand s\n"
    );

    let repeat_count = 200;

    // Account for base + variations + periodic comments
    let mut input = String::with_capacity(base.len() * repeat_count + repeat_count * 64);

    for i in 0..repeat_count {
        input.push_str(base);

        // ---- Controlled variability (deterministic) ----

        // Identifier variation
        input.push_str("| where ");
        input.push_str(match i % 5 {
            0 => "x",
            1 => "y",
            2 => "z",
            3 => "s",
            _ => "b",
        });

        // Numeric shape variation
        match i % 4 {
            0 => input.push_str(&format!(" > {}\n", i)),
            1 => input.push_str(&format!(" >= {}L\n", i * 10)),
            2 => input.push_str(&format!(" < {:.3}\n", i as f64 * 1.1)),
            _ => input.push_str(" != 0\n"),
        }

        // String length variation
        if i % 7 == 0 {
            input.push_str("| extend msg = \"short\"\n");
        } else if i % 7 == 1 {
            input.push_str("| extend msg = \"a somewhat longer string literal\"\n");
        } else if i % 7 == 2 {
            input.push_str("| extend msg = @\"verbatim string with \"\"quotes\"\"\"\n");
        }

        // Periodic comment to break patterns
        if i % 10 == 0 {
            input.push_str("// periodic comment\n");
        }
    }

    input
}

fn token_parser_tests(c: &mut Criterion) {
    let options = ParseOptions::default();

    // ---- Short input ----
    let short_input = "let x = 1;\n";
    let mut short = c.benchmark_group("token_parser/short");

    short.throughput(Throughput::Bytes(short_input.len() as u64));
    short.bench_function("parse", |b| {
        b.iter(|| {
            let token_stream = parse_tokens(black_box(short_input), black_box(&options));
            black_box(token_stream.tokens);
            black_box(token_stream.source);
        });
    });
    short.finish();

    // ---- Long input ----
    let long_input = build_long_input();
    let mut long = c.benchmark_group("token_parser/long");

    long.throughput(Throughput::Bytes(long_input.len() as u64));
    long.bench_function("parse", |b| {
        b.iter(|| {
            let token_stream = parse_tokens(black_box(&long_input), black_box(&options));
            black_box(token_stream.tokens);
            black_box(token_stream.source);
        });
    });
    long.finish();
}

criterion_group!(token_parser_benches, token_parser_tests);
