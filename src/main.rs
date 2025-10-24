// use std::io::Write as _;
use std::fmt::Write as _;

use i2::{lexer, parser};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).expect("input file not specified");
    let mut fnf_msg = String::new();
    write!(&mut fnf_msg, "file not found: {}", file_path).expect("file not found");
    let input = std::fs::read_to_string(file_path).expect(&fnf_msg);
    println!("{input}");
    let tokens = lexer::lex(input);
    let mut line = 1;
    for t in &tokens {
        if line != t.line {
            // for _ in 0..2*(t.line-line) {
            //     println!();
            // }
            println!();
            println!();
            line = t.line
        }
        print!("{t:?} ");
    }
    println!();
    println!();
    let tree = parser::parse(tokens);
    println!("{tree:?}");
}
