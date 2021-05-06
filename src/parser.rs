use crate::lexer::{Token, MOCK_IDX};
use std::iter::Peekable;
use std::slice::Iter;
use crate::lexer::Token::{RBracket, Semicolon};

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
    VarDeclStmt(String),

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

fn consume<'a>(iterator: &'a mut Peekable<Iter<Token>>, expected:&Token) -> Result<&'a Token, String> {
    let token = match iterator.next() {
        Some(t) => {t}
        None => {return Err("unexpected end".to_string())}
    };

    if std::mem::discriminant(token)!=std::mem::discriminant(expected){
        return Err(format!("Expected {}, got {}", expected.get_token_type_name(), token));
    }
    return Ok(token);
}

fn consume_msg<'a>(iterator: &'a mut Peekable<Iter<Token>>, expected:&Token, error_msg:String) -> Result<&'a Token, String> {
    let token = match iterator.next() {
        Some(t) => {t}
        None => {return Err("unexpected end".to_string())}
    };

    if std::mem::discriminant(token)!=std::mem::discriminant(expected){
        return Err(error_msg);
    }
    return Ok(token);
}

fn term(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    if let Some(token) = iterator.peek(){
        match token{
            Token::Number(i,_) => {
                let mut tmp = Expr::new();
                tmp.expr_type = ExprType::Literal(*i);
                iterator.next();
                return Ok(tmp);
            }

            Token::Identifier(name, _) => {
                let mut tmp = Expr::new();
                tmp.expr_type = ExprType::Variable(name.clone());
                iterator.next();

                return Ok(tmp);
            }

            Token::LBracket(r) => {
                iterator.next();
                let expr = expr(iterator)?;

                consume_msg(iterator, &RBracket(MOCK_IDX), format!("expected ')' for opening '(' at {}", r))?;
                return Ok(expr);
            }
            r => {
                return Err(format!("unexpected token {}", r));}
        }
    }else{
        return Err("unexpected end".to_string());
    }

}

fn addition(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut left_node = mult(iterator)?;
    while let Some(token) = iterator.peek() {
        match token {
            Token::Op(c, _) if *c=='+'||*c=='-' => {
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
            Token::Op(c, _) if *c=='*' || *c=='/' => {
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
    let print_kwrd = iterator.next().unwrap(); //consume print

    let mut res = Expr::new();
    res.expr_type = ExprType::PrintStmt;
    let sub = expr(iterator);
    let sub = match sub {
        Ok(e) => {e}
        Err(s) => {return Err(s + "\n" + &*format!("expected expression after print at {}", print_kwrd.get_pos()))}
    };

    consume(iterator, &Semicolon(MOCK_IDX))?;

    res.children.push(sub);
    return Ok(res);

}

fn var_decl_stmt(iterator:&mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    iterator.next(); //consume var
    let var_name = match consume(iterator, &Token::Identifier("".to_string(), MOCK_IDX)) {
        Ok(t) => {match t{
            Token::Identifier(s, _) => {s.clone()}
            _ => {return Err("error parsing variable name".to_string())} //shouldn't happen
        }}

        Err(msg) => {
            return Err(msg);
        }
    };

    let mut res = Expr{expr_type:ExprType::VarDeclStmt(var_name), children:Vec::new()};
    match iterator.peek(){
        Some(Token::Equals(_)) => {
            iterator.next(); // consume =
            let assignee = expr(iterator)?;
            res.children.push(assignee);
        }
        _ => {}
    }
    consume(iterator, &Token::Semicolon(MOCK_IDX))?;
    return Ok(res);
}


fn assign_stmt(iterator:&mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    //let var_name = iterator.next(); //take and cosume name
    let var_name = match consume(iterator, &Token::Identifier("".to_string(), MOCK_IDX))  {
        Ok(r) => {
            match r {
                Token::Identifier(a, _) => {a.clone()}
                _ => {return Err("error getting variable name".to_string())}
            }
        }
        Err(msg) => {return Err(msg)}
    };

    let eq_idx = consume(iterator, &Token::Equals(MOCK_IDX))?.get_pos();

    let sub = match expr(iterator) {
        Ok(r) => {r}
        Err(msg) => {return Err(msg + "\n" + &*format!("expected expression in assignment at {}\n", eq_idx))}
    };

    consume(iterator, &Token::Semicolon(MOCK_IDX))?;

    let mut res = Expr::new();
    res.expr_type = ExprType::AssignStmt(var_name);
    res.children.push(sub);
    return Ok(res);
}

fn stmt(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    if let Some(token) = iterator.peek() {
        match *token {
            Token::Print(_) => { return print_stmt(iterator); }
            Token::Var(_) => {return var_decl_stmt(iterator); }
            Token::Identifier(..) => { return assign_stmt(iterator); }
            r => {return Err(format!("unexpected token {}", r))}
        }
    }else{
        return Err("unexpected end of string".to_string());
    }
}

fn program(iterator: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut res = Expr::new();
    res.expr_type = ExprType::Program;
    let mut had_error = false;
    let mut err_msg = String::new();
    while let Some(x) = iterator.peek() {
        match x {
            Token::EOF(..) => {break;}
            _ => {
                let expr_ = match stmt(iterator){
                    Ok(t) => {t}
                    Err(msg) => { //error parsing
                        had_error = true;
                        err_msg.push('\n');
                        err_msg.push_str(&*msg);
                        //synchronise, report error later
                        while let Some(x) = iterator.next() {
                            match x{ //read till semicolon, continue as if nothing happened
                                Token::Semicolon(_) => {break;}
                                _ =>{}
                            }
                        }
                        continue;
                    }
                };
                res.children.push(expr_);}
        }

    }
    if had_error {
        return Err(err_msg);
    }

    return Ok(res);
}

pub fn parse(tokens:&Vec<Token>) -> Result<Expr, String> {
    let mut iterator: Peekable<Iter<Token>> = tokens.iter().peekable();
    program(&mut iterator)
}