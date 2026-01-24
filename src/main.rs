use chumsky::prelude::*;
use kusto_language::{
    syntax_parser::query,
    token_parser::{ParseOptions, parse_tokens},
};

fn main() {
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let input = "{ \"key\": true, 'array': [1, 2, 3], \"nested\": { 'innerKey': -3.14 } }";
    let tokens = parse_tokens(input, &options);
    let result = query(input).parse(&tokens);

    if let Some(syntax_node) = result.clone().into_output() {
        println!("{:#?}", syntax_node);
    }

    if result.has_errors() {
        for err in result.into_errors() {
            println!("Error: {:?}", err);
        }
    }
}
