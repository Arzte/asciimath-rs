// mod lib;
// use lib::parser::{parse, Evaluate, Scope};
extern crate asciimath;
use asciimath::Evaluate;

fn main() {
    let expression =
        asciimath::parse("abcx + 4 * 2 / ( 1 - 5 ) ^ (2 - 2 ^ 3)").unwrap();

    let mut scope = asciimath::Scope::new();
    scope.set_var("abcx", 3);

    println!("{:?}", expression.eval_with(&scope));
}
