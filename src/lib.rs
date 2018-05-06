#[macro_use] extern crate lazy_static;
extern crate lalrpop_util;

mod grammar;
pub mod types;
mod eval;
mod error;

pub use types::{Context, Value, Expression};

pub type ParseError<'a> = lalrpop_util::ParseError<usize, grammar::Token<'a>, &'static str>;

lazy_static! {
    static ref PARSER: grammar::ExprParser = grammar::ExprParser::new();
}

pub fn parse<'i>(input: &'i str) -> Result<types::Expression, ParseError> {
    PARSER.parse(input)
}
