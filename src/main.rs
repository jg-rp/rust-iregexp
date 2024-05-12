use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "iregexp.pest"]
pub struct IRegexp;

fn main() {
    let successful_parse = IRegexp::parse(Rule::pattern, r"\p{Ll}");
    println!("{:?}", successful_parse);

    let unsuccessful_parse = IRegexp::parse(Rule::pattern, r"\S+");
    println!("{:?}", unsuccessful_parse);
}

// TODO: cli
// TODO: lib
// TODO: Python bindings
// TODO: tests
