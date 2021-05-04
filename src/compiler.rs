use crate::vm::OpCode;
use crate::parser::{Expr, ExprType};
use std::collections::{HashMap};

pub struct Chunk{
    pub program:Vec<OpCode>,
    pub variable_size:usize,

    pub constant_pool: Vec<i32>
}

impl Chunk{
    pub fn new() -> Chunk{
        return Chunk{program:Vec::new(), variable_size:0, constant_pool:Vec::new()};
    }

    pub fn dump_stdout(&self){
        println!("constant_size={}\nvariable_size={}", self.constant_pool.len(), self.variable_size);
        for item in &self.program {
            println!("{}", item);
        }
    }

    pub fn compile_from(ast:&Expr) -> Result<Chunk, String> {
        Compiler::compile(ast)
    }
}

pub struct Compiler {
    name_map:HashMap<String, usize>
}

impl Compiler {

    pub fn new() -> Compiler {
        Compiler{name_map:HashMap::new()}
    }

    pub fn compile(ast:&Expr) -> Result<Chunk, String> {

        let mut name_map:HashMap<String, usize> = HashMap::new(); //variable -> variable_idx
        let mut variable_size= 0usize;
        Compiler::find_variables(ast, &mut name_map, &mut variable_size)?;

        let mut code_chunk = Chunk::new();
        code_chunk.variable_size = name_map.len();

        let mut comp = Compiler{ name_map };
        comp.compile_ast(&mut code_chunk, ast)?;

        Ok(code_chunk)
    }

    pub fn continue_compile(&mut self, ast:&Expr) -> Result<Chunk, String> {
        let name_map_copy = self.name_map.clone();
        //we don't want bad input to spoil compiler state
        //make copy to recover if needed


        return match self._continue_compile(ast) {
            Ok(chunk) => {
                Ok(chunk)
            }
            Err(r) => {
                //recover
                self.name_map = name_map_copy;
                Err(r)
            }
        }

    }


    fn _continue_compile(&mut self, ast:&Expr) -> Result<Chunk, String> {
        //has name_map
        let mut variable_size = 0usize;

        Compiler::find_variables(ast, &mut self.name_map, &mut variable_size)?;

        let mut code_chunk = Chunk::new();
        code_chunk.variable_size = variable_size;
        self.compile_ast(&mut code_chunk, ast)?;

        Ok(code_chunk)
    }



    fn find_variables<'a>(ast:&'a Expr, names:&mut HashMap<String, usize>, variable_counter: &mut usize) -> Result<(), String>{
        /*
        builds variable index & checks for name errors
         */
        match &ast.expr_type {
            ExprType::AssignStmt(variable_name) => {
                return if !names.contains_key(variable_name.as_str()) {
                    Err(format!("undeclared variable {}", variable_name))
                } else {
                    Ok(())
                }
            }

            ExprType::VarDeclStmt(variable_name) => {
                if !ast.children.is_empty() {
                    Compiler::find_variables(ast.children.first().unwrap(),
                    names, variable_counter)?;
                }


                if names.contains_key(variable_name.as_str()){
                    return Err(format!("redefinition of variable {}", variable_name));
                }else{
                    let idx = *variable_counter;
                    names.insert(variable_name.to_string(), idx);
                    *variable_counter += 1;
                    return Ok(());
                }
            }

            _ => {
                for item in ast.children.as_slice() {
                    Compiler::find_variables(item, names, variable_counter)?;
                }
            }
        }
        return Ok(());
    }

    fn push_extensions(code_chunk:&mut Chunk, addr:usize){
        if addr<= 0xff {
            return;
        }

        let arr: Vec<u8> = addr.to_be_bytes().to_vec();
        let arr:Vec<u8> = arr.into_iter().skip_while(|x| *x==0u8).collect();
        let arr = arr.split_last().unwrap().1;

        for x in arr {
            code_chunk.program.push(OpCode::Extend(*x));
        }

    }

    fn compile_ast(&mut self, code_chunk:&mut Chunk,  ast: &Expr) -> Result<(), String>{
        match &ast.expr_type {
            ExprType::Op(c) => {
                self.compile_ast(code_chunk,  &ast.children[0])?;
                self.compile_ast(code_chunk,  &ast.children[1])?;

                match c {
                    '+' => {code_chunk.program.push(OpCode::Add)}
                    '-' => {code_chunk.program.push(OpCode::Sub)}
                    '*' => {code_chunk.program.push(OpCode::Mult)}
                    '/' => {code_chunk.program.push(OpCode::Div)}
                    _ => {} //wont happen (hopefully)
                }
            }

            ExprType::Literal(i) => {
                let idx = code_chunk.constant_pool.len();
                code_chunk.constant_pool.push(*i);
                Compiler::push_extensions(code_chunk, idx);
                code_chunk.program.push(OpCode::LoadConst(idx as u8));

            }
            ExprType::Variable(name) => {
                let idx = match self.name_map.get(name.as_str()){
                    None => {return Err(format!("unknown variable {}", name));}
                    Some(x) => {*x}
                };

                Compiler::push_extensions(code_chunk, idx);
                code_chunk.program.push(OpCode::LoadVar(idx as u8 ));
            }

            ExprType::AssignStmt(name) => {
                //we know that names are fine resolved
                let idx:usize = *self.name_map.get(name.as_str()).unwrap();

                self.compile_ast(code_chunk, &ast.children[0])?;

                code_chunk.program.push(OpCode::Store(idx as u8));
            }

            ExprType::PrintStmt => {
                self.compile_ast(code_chunk, &ast.children[0])?;
                code_chunk.program.push(OpCode::Print);
            }
            ExprType::Program => {
                for stmt in &ast.children{
                    self.compile_ast(code_chunk, stmt)?;
                }
            }

            ExprType::VarDeclStmt(varname) => {
                if !ast.children.is_empty() { // has initializer
                    self.compile_ast(code_chunk, ast.children.first().unwrap())?;
                    let idx:usize = *self.name_map.get(varname).unwrap();
                    Compiler::push_extensions(code_chunk, idx);
                    code_chunk.program.push(OpCode::Store(idx as u8));
                }
            }
        }
        return Ok(());
    }

}