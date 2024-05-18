use std::{
    fmt::Display, iter::Peekable,str::Chars,
};

pub type Result<T> = std::result::Result<T, ExpError>;

#[derive(Debug)]
pub enum ExpError {
    Parse(String),
}

impl std::error::Error for ExpError {}

impl Display for ExpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Copy)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divider,
    Power,
    LeftParen,
    RightParen,
}

const ASSOC_LEFT: i32 = 0;
const ASSOC_RIGHT: i32 = 1;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Number(n) => n.to_string(),
                Token::Plus => '+'.to_string(),
                Token::Minus => '-'.to_string(),
                Token::Multiply => '*'.to_string(),
                Token::Divider => '/'.to_string(),
                Token::Power => '^'.to_string(),
                Token::LeftParen => '('.to_string(),
                Token::RightParen => ')'.to_string(),
            }
        )
    }
}

impl Token {
    fn is_operator(&self) -> bool {
        match self {
            Token::Plus | Token::Minus | Token::Multiply | Token::Divider | Token::Power => true,
            _ => false,
        }
    }

    fn precedence(&self) -> i32 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divider => 2,
            Token::Power => 3,
            _ => 0,
        }
    }

    fn assoc(&self) -> i32 {
        match self {
            Token::Power => ASSOC_RIGHT,
            _ => ASSOC_LEFT,
        }
    }

    fn compute(&self, l: i32, r: i32) -> Option<i32> {
        match self {
            Token::Plus => Some(l + r),
            Token::Minus => Some(l - r),
            Token::Multiply => Some(l * r),
            Token::Divider => Some(l / r),
            Token::Power => Some(l.pow(r as u32)),
            _ => None,
        }
    }
}

struct Tokenize<'a> {
    tokens: Peekable<Chars<'a>>,
}

impl<'a> Tokenize<'a> {
    fn new(expr: &'a str) -> Self {
        Self {
            tokens: expr.chars().peekable(),
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.tokens.peek() {
            if c.is_whitespace() {
                self.tokens.next();
            } else {
                break;
            }
        }
    }

    fn scan_number(&mut self) -> Option<Token> {
        let mut num = String::new();
        while let Some(&c) = self.tokens.peek() {
            if c.is_numeric() {
                num.push(c);
                self.tokens.next();
            } else {
                break;
            }
        }

        match num.parse() {
            Ok(n) => Some(Token::Number(n)),
            Err(_) => None,
        }
    }

    fn scan_operator(&mut self) -> Option<Token> {
        match self.tokens.next() {
            Some('+') => Some(Token::Plus),
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divider),
            Some('^') => Some(Token::Power),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            _ => None,
        }
    }
}
impl<'a> Iterator for Tokenize<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_whitespace();
        match self.tokens.peek() {
            Some(c) if c.is_numeric() => self.scan_number(),
            Some(_) => self.scan_operator(),
            None => return None,
        }
    }
}

struct Expr<'a> {
    iter: Peekable<Tokenize<'a>>,
}

impl<'a> Expr<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: Tokenize::new(src).peekable(),
        }
    }

    pub fn eval(&mut self) -> Result<i32> {
        let result = self.compute_expr(1)?;

        if self.iter.peek().is_some() {
            return Err(ExpError::Parse("Unexpcedted end of expr".into()));
        }
        Ok(result)
    }

    fn compute_atom(&mut self) -> Result<i32> {
        match self.iter.peek() {
            Some(Token::Number(n)) => {
                let val = *n;
                self.iter.next();
                return Ok(val);
            }
            Some(Token::LeftParen) => {
                self.iter.next();
                let result = self.compute_expr(1)?;
                match self.iter.next() {
                    Some(Token::RightParen) => (),
                    _ => return Err(ExpError::Parse("Unexpected charcter".into())),
                }
                return Ok(result);
            }
            _ => {
                return Err(ExpError::Parse(
                    "Expecting a number of left parenthesis".into(),
                ))
            }
        }
    }

    fn compute_expr(&mut self, min_prec: i32) -> Result<i32> {
        let mut atom_lhs = self.compute_atom()?;

        loop {
            let cur_token = self.iter.peek();
            if cur_token.is_none() {
                break;
            }
            let token = *cur_token.unwrap();

            if !token.is_operator() || token.precedence() < min_prec {
                break;
            }

            let mut next_prec = token.precedence();
            if token.assoc() == ASSOC_LEFT {
                next_prec += 1;
            }

            self.iter.next();

            let atom_rhs = self.compute_expr(next_prec)?;
            match token.compute(atom_lhs, atom_rhs) {
                Some(res) => atom_lhs = res,
                None => return Err(ExpError::Parse("Unexpected expr".into())),
            }
        }
        Ok(atom_lhs)
    }
}

fn main() {
    let src = "11 + 32 + 5 * 19 - (19 - 10) / 4 + 45";
    let mut expr = Expr::new(src);
    let result = expr.eval();
    println!("res = {:?}", result);
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_number() {
        let src = "100";
        let mut expr = Expr::new(src);
        let result = expr.eval().unwrap();
        assert_eq!(100, result);
    }

    #[test]
    fn test_add() {
        let src = "100 + 98 + 19 + 98";
        let mut expr = Expr::new(src);
        let result = expr.eval().unwrap();
        assert_eq!(100 + 98 + 19 + 98, result);
    }

    #[test]
    fn test_minus() {
        let src = "100 - 90 - 10 - 100";
        let mut expr = Expr::new(src);
        let result = expr.eval().unwrap();
        assert_eq!(100 - 90 - 10 - 100, result);
    }

    #[test]
    fn test_multiply() {
        let src = "100 * 9 *5 * 2";
        let mut expr = Expr::new(src);
        let result = expr.eval().unwrap();
        assert_eq!(100 * 9 * 5 * 2, result);
    }
}