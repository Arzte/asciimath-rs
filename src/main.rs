// mod lib;
// use lib::parser::{parse, Evaluate, Scope};
extern crate asciimath;
use asciimath::Evaluate;

fn main() {
    // let expr = parse_expr("1*2*3 + 24*5 - 10");
    // println!("{:?}", expr);
    // println!("{}", &expr.eval());3 + 4 * 2 / ( 1 − 5 ) ^ 2 ^ 3
    let mut scope = asciimath::Scope::new();
    scope.set_var("x", 3);

    println!(
        "{:?}",
        asciimath::parse("x + 4 * 2 / ( 1 - 5 ) ^ (2 - 2 ^ 3)")
            .eval_with(&scope)
    );
}
