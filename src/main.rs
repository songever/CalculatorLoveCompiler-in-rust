// Calculator love compiler (^ 3 ^) !
// But in rust!!
// [Inspired by VectorizeOrz]

/* eg. 
1+5*3 
-13/5+2*7+8
(1+3)*(1024-1024)
18/(-3+9)
123/0
*/

use std::{iter::Peekable, str::Chars};

/* ENBF Grammar
<Expr> ::= <AddSubExpr>                                    Priority |  LOW
<AddSubExpr> ::= <MulDivExpr> {('+'|'-') <MulDivExpr>}              |
<MulDivExpr> ::= <PrimaryExpr> {('*'|'/') <PrimaryExpr>}            |
<PrimaryExpr> ::= NUM | '-'NUM |'('<Expr>')'                       \|/ HIGH
*/
const DEBUG: i32 = 1;
fn debug(msg: &str) {
    if DEBUG != 0 {
        println!("[debug]: {msg}");
    }
}
// --- Lexer --- 
enum TokenType
{
    NUMBER(i32),
    ADD,SUB,MUL,DIV,
    LEFTPAREN, RIGHTPAREN,
    END
}

struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
}
impl<'a> From<&'a str> for Lexer<'a> {
    fn from(value: &'a str) -> Self {
        Lexer { src: value.chars().peekable() }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        match self.src.next() {
            Some('+') => { debug("TOKEN:ADD"); Some(TokenType::ADD)},
            Some('-') => { debug("TOKEN:SUB"); Some(TokenType::SUB)},
            Some('*') => { debug("TOKEN:MUL"); Some(TokenType::MUL)},
            Some('/') => { debug("TOKEN:DIV"); Some(TokenType::DIV)},
            Some('(') => { debug("TOKEN:LEFTPAREN"); Some(TokenType::LEFTPAREN)},
            Some(')') => { debug("TOKEN:RIGHTPAREN"); Some(TokenType::RIGHTPAREN)},
            Some(head@'0'..='9') => {
                debug("TOKEN:NUMBER");
                let mut num = head.to_digit(10).unwrap() as i32;
                while let Some(&following@'0'..='9') = self.src.peek() {
                    num = num * 10 + following.to_digit(10).unwrap() as i32;
                    self.src.next();
                }
                Some(TokenType::NUMBER(num))
            }
            None => {debug("TOKEN: END"); Some(TokenType::END) },
            _ => None,
        }
    }
}
// --- Parser & Interpreter --- (both in EvalXXX) 
//<Expr> ::= <AddSubExpr>
fn eval_expr(lexer: &mut Lexer) -> i32
{
    // Debug("EVAL:Expr");

    eval_add_sub_expr(lexer)
}

//<AddSubExpr> ::= <MulDivExpr> {('+'|'-') <MulDivExpr>}
fn eval_add_sub_expr(lexer: &mut Lexer) -> i32
{
    debug("EVAL:AddSubExpr");

    let mut addsub_exprval = eval_mul_div_expr(lexer);
    while let Some('+') | Some('-') = lexer.src.peek() // if have + or - go on
    {
        match lexer.next() {
            Some(TokenType::ADD) => addsub_exprval += eval_mul_div_expr(lexer),
            Some(TokenType::SUB) => addsub_exprval -= eval_mul_div_expr(lexer),
            _ => unreachable!(),
        }
    }

    addsub_exprval
}

//<MulDivExpr> ::= <PrimaryExpr> {('*'|'/') <PrimaryExpr>}
fn eval_mul_div_expr(lexer: &mut Lexer) -> i32
{
    debug("EVAL:MulDivExpr");

    let mut muldiv_exprval = eval_primary_expr(lexer);
    while let Some('*') | Some('/') = lexer.src.peek() // if have * or / go on
    {
        match lexer.next() {
            Some(TokenType::MUL) => muldiv_exprval *= eval_primary_expr(lexer),
            Some(TokenType::DIV) => {
                match eval_primary_expr(lexer) {
                    0 => panic!("Division by zero"), 
                    nonzero => muldiv_exprval /= nonzero,
                };
            }
            _ => unreachable!(),
        }
    }
    muldiv_exprval
}

//<PrimaryExpr> ::= NUM | '-'NUM | '('<Expr>')'
fn eval_primary_expr(lexer: &mut Lexer) -> i32
{
    debug("EVAL:PrimaryExpr");

    match lexer.next() {
        Some(TokenType::NUMBER(num)) => num,
        Some(TokenType::SUB) => {
            if let Some('0'..='9') = lexer.src.peek() {
                match lexer.next() {
                    Some(TokenType::NUMBER(num)) => -num,
                    _ => panic!("Negative sign without a number"),
                }
            } else {
                panic!("Negative sign without a number");
            }
        },
        Some(TokenType::LEFTPAREN) => {
            let num = eval_expr(lexer);
            match lexer.next() {
                Some(TokenType::RIGHTPAREN) => num,
                _ => panic!("Mismatched parentheses"),
            }
        }
        _ => panic!("Illegal PrimaryExpr"),
    }
}

// --- Driver ---
fn main() {
    let mut src = String::new();
    std::io::stdin().read_line(&mut src).expect("Failed to read input");
    let mut lexer = Lexer::from(src.as_ref());

    let expr_val = eval_expr(&mut lexer);
    println!("\nEvaluate result :)");
    println!("{expr_val}");
}
