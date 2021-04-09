use crate::vm::OpCode;
use crate::parser::{Expr, ExprType};
use std::collections::HashMap;

pub struct Chunk{
    pub program:Vec<OpCode>,
    pub constant_size:usize,
    pub variable_size:usize,
}

impl Chunk{
    pub fn new() -> Chunk{
        return Chunk{program:Vec::new(), constant_size:0, variable_size:0};
    }

    pub fn dump_stdout(&self){
        for item in &self.program {
            println!("{}", item);
        }
    }

    pub fn compile_from(&mut self, ast:&Expr) -> Result<Vec<i32>, String> {
        self.find_constants(ast);
        self.find_variables(ast);

        let mut value_stack = vec![0;self.constant_size+self.variable_size];

        let mut compiler = Compiler{constant_size:self.constant_size, variable_size:self.variable_size,
        constant_counter:0, variable_counter:0, name_map:HashMap::new()};


        self.compile(&mut value_stack, &mut compiler, ast)?;

        return Ok(value_stack);
    }

    fn find_constants(&mut self, ast:&Expr){
        match ast.expr_type {
            ExprType::Literal(_) => {self.constant_size+=1; return;}
            _ => {
                for item in ast.children.as_slice() {
                    self.find_constants(item);
                }
            }
        }
    }

    fn find_variables(&mut self, ast:&Expr){
        match ast.expr_type {
            ExprType::Variable(_) => {self.variable_size+=1; return;}
            _ => {
                for item in ast.children.as_slice() {
                    self.find_constants(item);
                }
            }
        }
    }

    fn compile<'c>(&mut self, value_stack:&mut Vec<i32>, compiler:&mut Compiler<'c>, ast: &'c Expr) -> Result<(), String>{
        match &ast.expr_type {
            ExprType::Op(c) => {
                self.compile(value_stack, compiler, &ast.children[0])?;
                self.compile(value_stack, compiler, &ast.children[1])?;

                match c {
                    '+' => {self.program.push(OpCode::Add)}
                    '-' => {self.program.push(OpCode::Sub)}
                    '*' => {self.program.push(OpCode::Mult)}
                    '/' => {self.program.push(OpCode::Div)}
                    _ => {} //wont happen (hopefully)
                }
            }
            ExprType::Literal(i) => {
                let idx = compiler.constant_counter;
                compiler.constant_counter+=1;
                value_stack[idx] = *i;
                self.program.push(OpCode::Load(idx as u8));

            }
            ExprType::Variable(name) => {
                let idx = compiler.name_map.get(name.as_str());
                if let None = idx {
                    return Err(format!("unknown variable {}", name));
                }
                self.program.push(OpCode::Load(*idx.unwrap() as u8));
            }

            ExprType::AssignStmt(name) => {

                let idx:usize = match compiler.name_map.get(name.as_str()) {
                    None => {
                        let idx = compiler.variable_counter;
                        compiler.name_map.insert(name.as_str(), idx);
                        compiler.variable_counter+=1;
                        idx

                    }
                    Some(idx) => {
                        *idx
                    }
                };

                self.compile(value_stack, compiler, &ast.children[0])?;

                self.program.push(OpCode::Store(idx as u8));
            }

            ExprType::PrintStmt => {
                self.compile(value_stack, compiler, &ast.children[0])?;
                self.program.push(OpCode::Print);
            }
            ExprType::Program => {
                for stmt in &ast.children{
                    self.compile(value_stack, compiler, stmt)?;
                }
            }
        }
        return Ok(());
    }


}

#[allow(unused, dead_code)]
pub struct Compiler<'c> {
    constant_size:usize,
    variable_size:usize,

    constant_counter:usize,
    variable_counter:usize,
    name_map:HashMap<&'c str, usize>
}