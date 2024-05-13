use pest::Parser;
pub mod parser;
pub use parser::{IRegexp, Rule};

// TODO: docs

pub fn check(pattern: &str) -> bool {
    IRegexp::parse(Rule::pattern, pattern).is_ok()
}
