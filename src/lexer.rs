use std::fmt::{Display, Formatter};
use std::fmt;

pub enum Token{
    Op(char),
    Number(i32),
    LBracket,
    RBracket,
    Print,
    Equals,
    Identifier(String),
    Semicolon
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Token::Op(c) => {format!("<operator {}>", c)}
            Token::Number(n) => {format!("<number {}>", n)}
            Token::LBracket => {"<(>".to_string()}
            Token::RBracket => {"<)>".to_string()}
            Token::Print => {"<print>".to_string()}
            Token::Equals => {"<=>".to_string()}
            Token::Identifier(name) => {format!("<variable {}>", name)}
            Token::Semicolon => {"<;>".to_string()}
        })
    }
}

pub fn tokenize(input:&str) -> Result<Vec<Token>, String>{

    use Token::*;

    let mut iterator = input.char_indices().peekable();
    let mut res:Vec<Token> = Vec::new();
    while let Some(pair) = iterator.peek() {
        let c = (*pair).1;
        match c {
            '+' | '-'| '*'|'/' => {res.push(Op(c)); iterator.next();}

            '(' => {res.push(LBracket); iterator.next();}
            ')' => {res.push(RBracket); iterator.next();}

            _ if ('1'..='9').contains(&c) => {
                let mut num = String::new();
                num.push(c);
                iterator.next();

                while let Some(c) = iterator.peek().map(|pair| (*pair).1)  {

                    if ('0'..='9').contains(&c) {
                        num.push(c);
                        iterator.next();
                    }else{
                        break;
                    }
                }

                res.push(Number(str::parse::<i32>(&*num).unwrap()));

            }

            _ if isalpha(c) => {
                let mut token = String::new();
                token.push(c);
                iterator.next();

                while let Some(c) = iterator.peek().map(|pair| (*pair).1){
                    if isalphanum(c) {
                        token.push(c);
                        iterator.next();
                    }else{
                        break;
                    }
                }

                match token.as_str() {
                    "print" => {
                        res.push(Print);
                    }
                    _ => {
                        res.push(Identifier(token));
                    }
                }

            }

            '=' => {res.push(Equals); iterator.next();}
            ';' => {res.push(Semicolon); iterator.next();}

            _ if c==' '|| c=='\n'|| c=='\t'||c=='\r' => {iterator.next();}

            _ => {return Err(format!("unknown character {} at {}", c, (*pair).0))}
        }
    }
    return Ok(res);
}

fn isalpha(c:char) -> bool {
    return ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c=='_';
}

fn isalphanum(c:char) -> bool {
    return isalpha(c) || ('0'..='9').contains(&c);
}