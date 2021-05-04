use crate::lexer::{tokenize, Token};
use std::env;
use std::fs;

mod lexer;
mod parser;
mod lisp_print;
mod compiler;
mod vm;

use crate::vm::VM;
use crate::compiler::{Chunk, Compiler};
use std::io::{BufRead, BufReader};

fn run_repl(){
    let stdin_buffer = BufReader::new(std::io::stdin());
    let mut stdin_iterator = stdin_buffer.lines();
    let mut compiler = Compiler::new();
    let mut vm = VM::new();

    println!("REPL\nto exit type 'exit'");
    loop{
        let inp_str = stdin_iterator.next().unwrap().unwrap();

        if inp_str.as_str().trim()=="exit" {
            break;
        }

        let s = inp_str.trim();
        let tokens: Vec<Token> = match tokenize(s)  {
            Ok(res) => {res}
            Err(msg) => {println!("{}", msg); continue;}
        };

        let ast = match parser::parse(&tokens) {
            Ok(res) => {res}
            Err(msg) => {println!("{}", msg); continue;}
        };

        #[cfg(debug_assertions)]
        lisp_print::visit(&ast); //won't be printed in release

        let code_chunk = compiler.continue_compile(&ast);
        let code_chunk = match code_chunk {
            Ok(value) => {value}
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };

        #[cfg(debug_assertions)]
        code_chunk.dump_stdout(); //won't be printed in release

        match vm.run(&code_chunk) {
            Ok(_) => {}
            Err(msg) => {
                println!("{}", msg);
            }
        }

    }


}


fn main() {

    let args:Vec<String> = env::args().collect();

    //args[0] - program name

    if args.len()==1 {
        run_repl();
        return;
    }

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

    /*
     println!("{}",
              tokens
                  .iter()
                  .map(|x| format!("{}", x))
                  .collect::<Vec<String>>()
                  .join(", ")
     );*/

    let ast = match parser::parse(&tokens) {
        Ok(res) => {res}
        Err(msg) => {println!("{}", msg); return;}
    };

    #[cfg(debug_assertions)]
    lisp_print::visit(&ast); //won't be printed in release

    let code_chunk = Chunk::compile_from(&ast);
    let code_chunk = match code_chunk {
        Ok(value) => {value}
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    #[cfg(debug_assertions)]
    code_chunk.dump_stdout(); //won't be printed in release

    let mut vm = VM::new();

    match vm.run(&code_chunk) {
        Ok(_) => {}
        Err(msg) => {
            println!("{}", msg);
        }
    }

}
