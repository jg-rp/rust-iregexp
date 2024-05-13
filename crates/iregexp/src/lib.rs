use pest::Parser;
pub mod parser;
pub use parser::{IRegexp, Rule};

pub fn check(pattern: &str) -> bool {
    IRegexp::parse(Rule::pattern, pattern).is_ok()
}
