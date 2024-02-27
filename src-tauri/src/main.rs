#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![process])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Integer(i32),
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    OpenParent,
    CloseParent,
    End,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Dev,
    Mul,
    Pow,
    Neg,
}

#[derive(Debug)]
enum Expression {
    Binary(Operator, Box<Expression>, Box<Expression>),
    Unary(Operator, Box<Expression>),
    Integer(i32),
    Arbitrary(Operator, Vec<Expression>),
}

fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut last_c: char = ' ';
    let mut last_number: String = String::new();
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            continue;
        }
        if c.is_digit(10) {
            last_number.push(c);
        } 
        else {
            if !last_number.is_empty() {
                tokens.push(Token::Integer(last_number.parse::<i32>().unwrap()));
                last_number.clear()
            }
        }
        if c == '+' {
            tokens.push(Token::Plus)
        }
        else if c == '-' {
            tokens.push(Token::Minus)
        }
        else if c == '/' {
            tokens.push(Token::Slash)
        }
        else if c == '*' {
            tokens.push(Token::Star)
        }
        else if c == '^' {
            tokens.push(Token::Power)
        }
        else if c == '(' {
            tokens.push(Token::OpenParent)
        }
        else if c == ')' {
            tokens.push(Token::CloseParent)
        }
        last_c = c;
    }
    if !last_number.is_empty() {
        tokens.push(Token::Integer(last_number.parse::<i32>().unwrap()));
    }
    tokens.push(Token::End);
    Ok(tokens)
}

struct Parser {
    current_index: usize,
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            current_index: 0,
            tokens: tokens,
        }
    }
    
    fn consume_token(&mut self) {
        self.current_index += 1;
    }

    fn current_token(&self) -> Result<Token, &str> {
        if self.current_index >= self.tokens.len() {
            return Err("Current index out of range");
        }
        Ok(self.tokens[self.current_index])
    }
    
    fn primary(&mut self) -> Result<Expression, &str> {
        let token = self.current_token().unwrap();
        self.consume_token();
        match token {
            Token::Integer(n) => Ok(Expression::Integer(n)),
            Token::OpenParent => {
                let expr = self.expression().unwrap();
                match self.current_token().unwrap() {
                    Token::CloseParent => Ok(expr),
                    _ => Err("syntaxerr"),
                }
            }
            Token::Minus => {
                let expr = self.factor().unwrap();
                Ok(Expression::Unary(
                    Operator::Neg,
                    Box::new(expr),
                ))
            }
            _ => {
                Err("syntaxerr")
            }
        }
    }

    fn factor(&mut self) -> Result<Expression, &str> {
        let expr = self.primary().unwrap();
        let token = self.current_token().unwrap();
        println!("{:?}", expr);
        println!("{:?}", token);
        match token {
            Token::Power => {
                self.consume_token();
                let rhs = self.factor().unwrap();
                Ok(Expression::Binary(
                    Operator::Pow,
                    Box::new(expr),
                    Box::new(rhs),
                ))
            }
            _ => {
                Ok(expr)
            }
        }
    }

    fn term(&mut self) -> Result<Expression, &str> {
        let mut expr = self.factor().unwrap();

        loop {
            let token = self.current_token().unwrap();
            match token {
                Token::Star => {
                    self.consume_token();
                    let rhs = self.factor().unwrap();
                    expr = Expression::Binary(
                        Operator::Mul,
                        Box::new(expr),
                        Box::new(rhs)
                    );
                }
                Token::Slash => {
                    self.consume_token();
                    let rhs = self.factor().unwrap();
                    expr = Expression::Binary(
                        Operator::Dev,
                        Box::new(expr),
                        Box::new(rhs)
                    );
                }
                _ => break,
            };
        }
        Ok(expr)
        
    }

    fn expression(&mut self) -> Result<Expression, &str> {
        let mut expr = self.term().unwrap();
        loop {
            let token = self.current_token().unwrap();
            match token {
                Token::Plus => {
                    self.consume_token();
                    let rhs = self.term().unwrap();
                    expr = Expression::Binary(
                        Operator::Add,
                        Box::new(expr),
                        Box::new(rhs)
                    );
                }
                Token::Minus => {
                    self.consume_token();
                    let rhs = self.term().unwrap();
                    expr = Expression::Binary(
                        Operator::Add,
                        Box::new(expr),
                        Box::new(rhs)
                    );
                }
                _ => break,
            };
        }
        Ok(expr)
    }
}

#[tauri::command]
fn process(input: &str) -> String {
    let tokens: Vec<Token> = tokenize(input).unwrap();
    // let s = format!("{:?}", tokens);
    // return s;
    let mut parser = Parser::new(tokens);
    let res = parser.expression().unwrap();
    let s = format !("{:?}", res);
    s
}