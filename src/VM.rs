pub enum OpCode{
    Add, Sub, Mult, Div,
    Store(u8), LoadVar(u8), LoadConst(u8),
    Extend(u8),
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
            OpCode::LoadVar(idx) => {format!("[LOAD_VAR {}]", idx)}
            OpCode::Print => {"[PRINT]".to_string()}
            OpCode::Extend(idx) => {format!("[EXTEND {}]", idx)}
            OpCode::LoadConst(idx) => {format!("[LOAD_CONST {}]", idx)}
        })
    }
}

use crate::compiler::Chunk;
use std::fmt::{Display, Formatter};
use std::fmt;

pub struct VM{
    pub stack:Vec<i32>,
    pub initial_stack_size:usize,

}

impl VM{
    pub fn new() -> VM {
        return VM{stack:Vec::new(),
            initial_stack_size: 0
        }
    }

    fn checked_stack_pop(&mut self) -> Option<i32>{
        if self.stack.len()==self.initial_stack_size {return None;} //underflow into constants
        return self.stack.pop();
    }

    fn reset_variable_stack(&mut self){
        self.stack.truncate(self.initial_stack_size);
    }

    pub fn run(&mut self, code_chunk:&Chunk) -> Result<(), String> {



        if code_chunk.variable_size>0 { //add variable storage if needed
            self.initial_stack_size+=code_chunk.variable_size;
            self.stack.append(&mut vec![0; code_chunk.variable_size]);
        }

        println!("VM: stack_size={}, stack.len()={}", self.initial_stack_size, self.stack.len());

        let mut ip = 0;

        let mut idx_register:usize = 0;

        let mut status = Ok(());

        while ip<code_chunk.program.len() {
            match code_chunk.program[ip] {
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
                    idx_register = (idx_register<<8) + i as usize;

                    self.stack[idx_register] = value;
                    idx_register = 0;
                }else{ status = Err("stack underflow".to_string()); break;}

                }
                OpCode::LoadVar(i) => {if i as usize>=self.initial_stack_size {status = Err("value indexation error".to_string()); break;}
                    idx_register = (idx_register<<8) + i as usize;
                    self.stack.push(self.stack[idx_register]);
                    idx_register = 0;
                }
                OpCode::Print => {
                    if let Some(value) = self.checked_stack_pop() {
                        println!("{}", value);
                    }else{
                        status = Err("stack underflow".to_string()); break;
                    }
                }
                OpCode::Extend(i) => {
                    idx_register = (idx_register<<8) + i as usize;
                }
                OpCode::LoadConst(i) => {
                    idx_register = (idx_register<<8) + i as usize;
                    self.stack.push(code_chunk.constant_pool[idx_register]);
                    idx_register = 0;
                }
            }
            ip+=1;
        }

        self.reset_variable_stack();
        return status;
    }
}