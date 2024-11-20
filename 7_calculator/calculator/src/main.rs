use std::io;
use std::io::BufRead;
use pest::Parser;
extern crate pest_derive;
use crate::parser::{parse_expr, MyParser, Rule};

mod parser;

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match MyParser::parse(Rule::equation, &line?) {
            Ok(mut pairs) => {
                let expr = parse_expr(pairs.next().unwrap().into_inner());
                println!("Result: {}", expr.evaluate());
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}