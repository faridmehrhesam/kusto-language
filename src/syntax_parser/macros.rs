#[macro_export]
macro_rules! parser_return {
    ($output:ty) => {
        impl Parser<'a, &'a [TokenKind], $output, extra::Err<Rich<'a, TokenKind>>> + Clone
    };
}
