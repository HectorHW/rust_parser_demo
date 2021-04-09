use crate::parser::{ExprType, Expr};

pub fn visit(item: &Expr){
    match &item.expr_type {
        ExprType::Op(c) => {
            print!("({} ", c);
            visit(&item.children[0]);
            print!(" ");
            visit(&item.children[1]);
            print!(")");
        }
        ExprType::Literal(i) => {print!("{}", i)}
        ExprType::Variable(name) => {print!("{}", name)}
        ExprType::PrintStmt => { print!("(print ");
            visit(&item.children[0]);
            print!(")");
        }
        ExprType::AssignStmt(name) => {
            print!("(= {} ", name);
            visit(&item.children[0]);
            print!(")");
        }
        ExprType::Program => {
            for stmt in &item.children{
                visit(stmt);
                println!();
            }
        }
    }
}