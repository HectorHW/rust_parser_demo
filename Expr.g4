grammar Expr;

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