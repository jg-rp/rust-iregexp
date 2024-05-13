use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "iregexp.pest"]
pub struct IRegexp;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    macro_rules! assert_valid {
        ($($name:ident: $value:expr,)*) => {
            mod valid {
                use super::*;
                $(
                    #[test]
                    fn $name() {
                        assert!(IRegexp::parse(Rule::pattern, $value).is_ok());
                    }
                )*
            }
        }
    }

    macro_rules! assert_invalid {
        ($($name:ident: $value:expr,)*) => {
            mod invalid {
                use super::*;
                $(
                    #[test]
                    fn $name() {
                        assert!(IRegexp::parse(Rule::pattern, $value).is_err());
                    }
                )*
            }
        }
    }

    assert_valid! {
        char_class: r"[0-9\.]",
        alternation: r"foo|bar",
        char_class_exact_quantity: r"[ab]{3}",
        char_class_negation: r"[^ab]",
    }

    assert_invalid! {
        named_group: r"(?<group>[a-z]*)",
        multi_char_escape: r"[\S ]",
        multi_char_escape_: r"\d",
    }
}
