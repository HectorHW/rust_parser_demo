grammar Expr;

program: stmt*;

stmt: print_stmt | assignStmt | var_stmt;

print_stmt: PRINT expr SEMICOLON;

assignStmt: IDENTIFIER '=' expr SEMICOLON;

var_stmt: VAR IDENTIFIER ('=' expr)? SEMICOLON;

expr: addition;

addition: mult (('+'|'-') mult)*;
mult: term (('*'|'/')term)*;
term: NUMBER | IDENTIFIER | '(' expr ')';

WS: (' '| '\t'| '\n') -> channel(HIDDEN);

NUMBER: [1-9][0-9]*;
PRINT: 'print';
SEMICOLON: ';';
VAR: 'var';

IDENTIFIER: [A-Za-z_][A-Za-z0-9_]*;
