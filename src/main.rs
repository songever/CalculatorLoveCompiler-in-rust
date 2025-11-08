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

use std::{borrow::Borrow, iter::Peekable, str::Chars};

/* ENBF Grammar
<Expr> ::= <AddSubExpr>                                    Priority |  LOW
<AddSubExpr> ::= <MulDivExpr> {('+'|'-') <MulDivExpr>}              |
<MulDivExpr> ::= <PrimaryExpr> {('*'|'/') <PrimaryExpr>}            |
<PrimaryExpr> ::= NUM | '-'NUM |'('<Expr>')'                       \|/ HIGH
*/
const DEBUG: i32 = 1;

// --- Lexer --- 
enum TokenType
{
    NUMBER(u32),
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
            Some('+') => Some(TokenType::ADD),
            Some('-') => Some(TokenType::SUB),
            Some('*') => Some(TokenType::MUL),
            Some('/') => Some(TokenType::DIV),
            Some('(') => Some(TokenType::LEFTPAREN),
            Some(')') => Some(TokenType::RIGHTPAREN),
            Some(head@'0'..='9') => {
                let mut num = head.to_digit(10).unwrap();
                while let Some(&following@'0'..='9') = self.src.peek() {
                    num = num * 10 + following.to_digit(10).unwrap();
                    self.src.next();
                }
                Some(TokenType::NUMBER(num))
            }
            None => Some(TokenType::END),
            _ => None,
        }
    }
}
// --- Parser & Interpreter --- (both in EvalXXX) 


// --- Driver ---
fn main() {
    let mut src = String::new();
    std::io::stdin().read_line(&mut src).expect("Failed to read input");
    
    get_token();
    let expr_val = EvalExpr();
    println!("\nEvaluate result :)");
    println!("{expr_val}");
}
