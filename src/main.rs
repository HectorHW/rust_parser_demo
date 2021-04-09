use crate::lexer::{tokenize, Token};
use std::env;
use std::fs;

mod lexer;
mod parser;
mod lisp_print;
mod compiler;
mod vm;

use crate::vm::VM;
use crate::compiler::Chunk;

fn main() {

    let args:Vec<String> = env::args().collect();

    //args[0] - program name

    if args.len()!=2 {
        println!("usage : exec.exe <filename>");
        return;
    }

    let filename = args.get(1).unwrap();

    let content = fs::read_to_string(filename).expect("failed to read file.");

    println!("{}", content);

    let s = content.trim();
    let tokens: Vec<Token> = match tokenize(s)  {
        Ok(res) => {res}
        Err(msg) => {println!("{}", msg); return;}
    };

     println!("{}",
              tokens
                  .iter()
                  .map(|x| format!("{}", x))
                  .collect::<Vec<String>>()
                  .join(", ")
     );

    let ast = match parser::parse(&tokens) {
        Ok(res) => {res}
        Err(msg) => {println!("{}", msg); return;}
    };


    lisp_print::visit(&ast);
    println!();

    let mut code_chunk = Chunk::new();
    let value_stack = code_chunk.compile_from(&ast);
    let value_stack = match value_stack {
        Ok(value) => {value}
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };


    code_chunk.dump_stdout();

    let mut vm = VM::new(value_stack, code_chunk);

    match vm.run() {
        Ok(_) => {}
        Err(msg) => {
            println!("{}", msg);
        }
    }

}
