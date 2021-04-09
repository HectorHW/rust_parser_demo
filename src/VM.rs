pub enum OpCode{
    Add, Sub, Mult, Div,
    Store(u8), Load(u8),
    Print
}

impl Display for OpCode{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            OpCode::Add => {"[ADD]".to_string()}
            OpCode::Sub => {"[SUB]".to_string()}
            OpCode::Mult => {"[MULT]".to_string()}
            OpCode::Div => {"[DIV]".to_string()}
            OpCode::Store(idx) => {format!("[STORE {}]", idx)}
            OpCode::Load(idx) => {format!("[LOAD {}]", idx)}
            OpCode::Print => {"[PRINT]".to_string()}
        })
    }
}

use crate::compiler::Chunk;
use std::fmt::{Display, Formatter};
use std::fmt;

pub struct VM{
    pub stack:Vec<i32>,
    pub initial_stack_size:usize,

    pub code_chunk:Chunk
}

impl VM{
    pub fn new(stack:Vec<i32>, code_chunk:Chunk) -> VM {
        return VM{stack:stack,
            initial_stack_size: code_chunk.constant_size+code_chunk.variable_size,
            code_chunk:code_chunk
        }
    }

    fn checked_stack_pop(&mut self) -> Option<i32>{
        if self.stack.len()==self.initial_stack_size {return None;} //underflow into constants
        return self.stack.pop();
    }

    fn reset_variable_stack(&mut self){
        self.stack.truncate(self.initial_stack_size);
    }

    pub fn run(&mut self) -> Result<(), String> {

        let mut ip = 0;

        let mut status = Ok(());

        while ip<self.code_chunk.program.len() {
            match self.code_chunk.program[ip] {
                OpCode::Add => {
                    let b = self.checked_stack_pop();
                    let a = self.checked_stack_pop();
                    match (a,b){
                        (Some(a), Some(b)) => {self.stack.push(a+b);}
                        _ => {status= Err("stack underflow".to_string()); break;}
                    }
                }
                OpCode::Sub => {
                    let b = self.checked_stack_pop();
                    let a = self.checked_stack_pop();
                    match (a,b){
                        (Some(a), Some(b)) => {self.stack.push(a-b);}
                        _ => {status= Err("stack underflow".to_string());break;}
                    }
                }
                OpCode::Mult => {
                    let b = self.checked_stack_pop();
                    let a = self.checked_stack_pop();
                    match (a,b){
                        (Some(a), Some(b)) => {self.stack.push(a*b);}
                        _ => {status = Err("stack underflow".to_string()); break;}
                    }
                }
                OpCode::Div => {
                    let b = self.checked_stack_pop();
                    let a = self.checked_stack_pop();
                    match (a,b){
                        (Some(a), Some(b)) => {
                            if b==0 {status =  Err("zero division".to_string()); break;}
                            self.stack.push(a/b);}
                        _ => {status = Err("stack underflow".to_string()); break;}
                    }
                }
                OpCode::Store(i) => {if let Some(value) = self.checked_stack_pop() {
                    self.stack[i as usize] = value;
                }else{ status = Err("stack underflow".to_string()); break;}

                }
                OpCode::Load(i) => {if i as usize>=self.initial_stack_size {status = Err("value indexation error".to_string()); break;}
                    self.stack.push(self.stack[i as usize]);
                }
                OpCode::Print => {
                    if let Some(value) = self.checked_stack_pop() {
                        println!("{}", value);
                    }else{
                        status = Err("stack underflow".to_string()); break;
                    }
                }
            }
            ip+=1;
        }

        self.reset_variable_stack();
        return status;
    }
}