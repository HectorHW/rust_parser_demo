use crate::lexer::Token;
use std::iter::Peekable;
use std::slice::Iter;

/*
GRAMMAR:

program: stmt*;

stmt: print_stmt | assignStmt;

print_stmt: PRINT expr SEMICOLON;

assignStmt: IDENTIFIER '=' expr SEMICOLON;

expr: addition;

addition: mult (('+'|'-') mult)*;
mult: term (('*'|'/')term)*;
term: NUMBER | IDENTIFIER | '(' expr ')';

WS: (' '| '\t'| '\n') -> channel(HIDDEN);

NUMBER: [1-9][0-9]*;
PRINT: 'print';
SEMICOLON: ';';
IDENTIFIER: [A-Za-z_][A-Za-z0-9_]*;

 */

pub enum ExprType {
    Op(char),
    Literal(i32),
    Variable(String),

    PrintStmt,
    AssignStmt(String),

    Program
}


pub struct Expr {
    pub expr_type:ExprType,
    pub children: Vec<Expr>
}

impl Expr{
    pub fn new() -> Expr{
        return Expr{
            children: Vec::new(),
            expr_type: ExprType::Literal(0)
        }
    }
}

fn term(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    if let Some(token) = iterator.peek(){
        match token{
            Token::Number(i) => {
                let mut tmp = Expr::new();
                tmp.expr_type = ExprType::Literal(*i);
                iterator.next();
                return Ok(tmp);
            }

            Token::Identifier(name) => {
                let mut tmp = Expr::new();
                tmp.expr_type = ExprType::Variable(name.clone());
                iterator.next();

                return Ok(tmp);
            }

            Token::LBracket => {
                iterator.next();
                let expr = expr(iterator);

                match iterator.next() {
                    None => {return Err("unexpected EOL.".to_string());}
                    Some(token) => {

                        match token{
                            Token::RBracket => {return expr;}
                            _ => {return Err("unterminated string".to_string())}
                        }
                    }
                }
            }
            _ => {return Err("unexpected token".to_string());}
        }
    }else{
        return Err("unexpected end".to_string());
    }

}

fn addition(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut left_node = mult(iterator)?;
    while let Some(token) = iterator.peek() {
        match token {
            Token::Op(c) if *c=='+'||*c=='-' => {
                iterator.next();
                let right_node = mult(iterator)?;
                let mut tmp = Expr::new();
                tmp.expr_type = ExprType::Op(*c);
                tmp.children.push(left_node);
                tmp.children.push(right_node);
                left_node = tmp;
            }
            _ => {break;}
        }
    }
    return Ok(left_node);

}

fn mult(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut left_node = term(iterator)?;
    while let Some(token) = iterator.peek(){
        match token {
            Token::Op(c) if *c=='*' || *c=='/' => {
                iterator.next();
                let right_node = term(iterator)?;
                let mut tmp = Expr::new();
                tmp.expr_type = ExprType::Op(*c);
                tmp.children.push(left_node);
                tmp.children.push(right_node);
                left_node = tmp;
            }
            _ => {break;}
        }
    }
    return Ok(left_node);
}

fn expr(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    addition(iterator)
}

fn print_stmt(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    iterator.next(); //consume print

    let mut res = Expr::new();
    res.expr_type = ExprType::PrintStmt;
    let sub = expr(iterator);
    let sub = match sub {
        Ok(e) => {e}
        Err(_) => {return Err("expected expression after print".to_string())}
    };

    match iterator.next() {
        Some(Token::Semicolon) => {}
        _ => {return Err("expected semicolon.".to_string())}
    }

    res.children.push(sub);
    return Ok(res);

}


fn assign_stmt(iterator:&mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let var_name = iterator.next(); //take and cosume name

    let var_name = match var_name.unwrap() {
        Token::Identifier(s) => {s.clone()}
        _ => {return Err("unexpected token".to_string())}
    };

    match iterator.next(){
        None => {return Err("unexpected EOF".to_string())}
        Some(token) => {
            match token{
                Token::Equals => {}
                _ => {return Err("unexpected token".to_string())}
            }
        }
    }

    let sub = expr(iterator)?;

    match iterator.next() {
        Some(Token::Semicolon) => {}
        _ => {return Err("expected semicolon.".to_string())}
    }

    let mut res = Expr::new();
    res.expr_type = ExprType::AssignStmt(var_name);
    res.children.push(sub);
    return Ok(res);
}

fn stmt(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    if let Some(token) = iterator.peek() {
        match *token {
            Token::Print => { return print_stmt(iterator); }
            Token::Identifier(_) => { return assign_stmt(iterator); }
            _ => {return Err("unexpected token.".to_string())}
        }
    }else{
        return Err("unexpected end of string".to_string());
    }
}

fn program(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut res = Expr::new();
    res.expr_type = ExprType::Program;
    while let Some(..) = iterator.peek() {
        res.children.push(stmt(iterator)?);
    }
    return Ok(res);
}

pub fn parse(tokens:&Vec<Token>) -> Result<Expr, String> {
    let mut iterator: Peekable<Iter<Token>> = tokens.iter().peekable();
    program(&mut iterator)
}