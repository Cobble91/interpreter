use std::env;
use std::io::prelude::*;
use std::process::exit;
use std::{fs::File, path::Path};

use interpreter::lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect number of arguments. Usage: int <file>");
        exit(1);
    }
    let path = Path::new(&args[1]);
    let mut file =  match File::open(path) {
        Err(why) => panic!("File \'{}\' not found: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut buffer = String::new();
    if let Err(why) = file.read_to_string(&mut buffer) {
        panic!("Unable to read file \'{}\': {}", path.display(), why)
    };

    lexer::lex(buffer);
}
