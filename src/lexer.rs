use std::fmt::{Display, Formatter};
use std::fmt;
use std::option::Option::Some;
#[derive(Copy, Clone)]
pub struct TokenIndex {
    pub index: usize,
    pub line_number:usize
}

impl Display for TokenIndex{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.line_number, self.index)
    }
}

pub const MOCK_IDX: TokenIndex = TokenIndex{index:0,line_number:0};


pub enum Token{
    Op(char, TokenIndex),
    Number(i32, TokenIndex),
    LBracket(TokenIndex),
    RBracket(TokenIndex),
    Print(TokenIndex),
    Var(TokenIndex),
    Equals(TokenIndex),
    Identifier(String, TokenIndex),
    Semicolon(TokenIndex),
    EOF(TokenIndex)
}

impl Token {
    pub fn get_pos(&self) -> TokenIndex {
        *match self {
            Token::Op(_, r) => {r}
            Token::Number(_, r) => {r}
            Token::LBracket(r) => {r}
            Token::RBracket(r) => {r}
            Token::Print(r) => {r}
            Token::Var(r) => {r}
            Token::Equals(r) => {r}
            Token::Identifier(_, r) => {r}
            Token::Semicolon(r) => {r}
            Token::EOF(r) => {r}
        }
    }

    pub fn get_token_type_name(&self) -> String {
        match self {
            Token::Op(..) => {"Binary operator"}
            Token::Number(..) => {"Number"}
            Token::LBracket(_) => {"("}
            Token::RBracket(_) => {")"}
            Token::Print(_) => {"print keyword"}
            Token::Var(_) => {"var keyword"}
            Token::Equals(_) => {"equals"}
            Token::Identifier(..) => {"identifier"}
            Token::Semicolon(_) => {"semicolon"}
            Token::EOF(_) => {"EOF"}
        }.to_string()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Token::Op(c, r) => {format!("<operator {} [{},{}]>", c, r.line_number, r.index)}
            Token::Number(n, r) => {format!("<Number {} [{},{}]>", n, r.line_number, r.index)}
            Token::LBracket(r) => {format!("<( [{}, {}]>", r.line_number, r.index)}
            Token::RBracket(r) => {format!("<) [{}, {}]>", r.line_number, r.index)}
            Token::Print(r) => {format!("<print [{}, {}]>", r.line_number, r.index)}
            Token::Equals(r) => {format!("<= [{}, {}]>", r.line_number, r.index)}
            Token::Identifier(name, r) => {format!("<variable {} [{},{}]>", name, r.line_number, r.index)}
            Token::Semicolon(r) => {format!("<; [{},{}]>", r.line_number, r.index)}
            Token::Var(r) => {format!("<var [{},{}]>", r.line_number, r.index)}
            Token::EOF(r) => {format!("<EOF [{}, {}]>", r.line_number, r.index)}
        })
    }
}

pub fn tokenize(input:&str) -> Result<Vec<Token>, String>{

    use Token::*;

    let mut line_start = 0;
    let mut line_number = 0;

    fn index(index:usize, line_number:usize) -> TokenIndex{
        TokenIndex{index, line_number}
    }

    fn current_index(absolute_idx: usize, line_start:usize, line_number:usize) -> TokenIndex {
        index(absolute_idx-line_start, line_number)
    }

    fn update_newline(absolute_idx:usize, line_start:&mut usize, line_number:&mut usize) {
        *line_number += 1;
        *line_start = absolute_idx+1;
    }


    let mut iterator = input.char_indices().peekable();
    let mut res:Vec<Token> = Vec::new();
    while let Some(pair) = iterator.peek() {
        let c = (*pair).1;
        let absolute_idx = (*pair).0;
        match c {
            '+' | '-'| '*' => {res.push(Op(c, current_index(pair.0, line_start, line_number))); iterator.next();}

            '/' => {
                //division or comment
                iterator.next(); //consume it
                match iterator.peek() {
                    Some((_, '/')) => {
                        //comment
                        while let Some(pair) = iterator.next() { //skip until EOL
                            if pair.1=='\n' {update_newline(pair.0, &mut line_start, &mut line_number);break;}
                        }
                    }

                    Some((_, '*')) => { // start of multiline comment. /*
                        //skip until */
                        let mut ended_flag = false;
                        let start = current_index(absolute_idx, line_start, line_number);
                        while let (Some(p1), Some(p2)) = (iterator.next(), iterator.peek())
                        {
                            if p1.1=='\n' {update_newline(p1.0, &mut line_start, &mut line_number);}
                            if p1.1=='*' && p2.1=='/' {ended_flag=true; break;}
                        }

                        if !ended_flag {return Err(format!("lexer error: unterminated multiline comment starting at {}", start))}
                        iterator.next(); //consume /
                    }

                    _ => {res.push(Op('/', current_index(absolute_idx, line_start, line_number)))}
                }
            }

            '(' => {res.push(LBracket(current_index(absolute_idx, line_start, line_number))); iterator.next();}
            ')' => {res.push(RBracket(current_index(absolute_idx, line_start, line_number))); iterator.next();}

            _ if isnum(c) => {
                let start_idx = current_index(absolute_idx, line_start, line_number);
                let mut num = String::new();
                num.push(c);
                iterator.next();

                while let Some(c) = iterator.peek().map(|pair| (*pair).1)  {

                    if isnum(c) {
                        num.push(c);
                        iterator.next();
                    }else{
                        break;
                    }
                }

                res.push(Number(str::parse::<i32>(&*num).unwrap(), start_idx));

            }

            _ if isalpha(c) => {
                let mut token = String::new();
                let start_idx = current_index(absolute_idx, line_start, line_number);
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
                        res.push(Print(start_idx));
                    }
                    "var" => {
                        res.push(Var(start_idx));
                    }

                    _ => {
                        res.push(Identifier(token, start_idx));
                    }
                }

            }

            '=' => {res.push(Equals(current_index(absolute_idx, line_start, line_number))); iterator.next();}
            ';' => {res.push(Semicolon(current_index(absolute_idx, line_start, line_number))); iterator.next();}

            _ if c==' '|| c=='\t'||c=='\r' => {iterator.next();}
            _ if c=='\n' => {
                update_newline(absolute_idx, &mut line_start, &mut line_number);
                iterator.next();
            }

            _ => {return Err(format!("unknown character {} at {}", c, (*pair).0))}
        }
    }
    res.push(Token::EOF(current_index(input.len(), line_start, line_number)));
    return Ok(res);
}

fn isalpha(c:char) -> bool {
    return ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c=='_';
}
fn isnum(c:char) -> bool {
    return ('0'..='9').contains(&c);
}

fn isalphanum(c:char) -> bool {
    return isalpha(c) || isnum(c)
}