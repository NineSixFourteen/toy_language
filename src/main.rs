use std::{env, fs};

use compiler::Compiler;
use parser::{tokenizer::{Tokenizer}, Parser};

#[allow(dead_code)]
pub mod stack_machine;
#[allow(dead_code)]
pub mod parser;
pub mod tests;
#[allow(dead_code)]
pub mod compiler;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name;
    println!("{} __ {}",args.len(),args.first().unwrap());
    if args.len() == 1 && args.first().unwrap().ends_with("\\toy_language.exe") {
        file_name = "exl.txt"
    } else {
        file_name = args.get(1).unwrap();
    }
    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
    let mut tokens = Tokenizer::new(&contents);
    tokens.tokenize();
    match Parser::parse(tokens.tokens) {
        Ok(x) => {
            let mut c = Compiler::compile(x);
            match c.eval() {
                Ok(_res) => {},
                Err(x) =>panic!("Error : {:?}",x ),
            }
        }
        Err(p) => println!("ParseError : {:?}",p),
    } 
}
