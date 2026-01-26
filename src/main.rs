use chumsky::prelude::*;
use kusto_language::{
    syntax_parser::query,
    token_parser::{ParseOptions, parse_tokens},
};

fn main() {
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens("let ['name'] = | count as total", &options);
    let result = query().parse(&tokens);

    if let Some(syntax_node) = result.clone().into_output() {
        println!("{:#?}", syntax_node);
    }

    if result.has_errors() {
        for err in result.into_errors() {
            println!("Error: {:?}", err);
        }
    }
}
