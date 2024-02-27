#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![process, test])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

enum CalculatorError {
    SyntaxError(&'static str),
    InternalError(&'static str),
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
}

impl Expression {
    fn eval(&self) -> Result<i32, CalculatorError> {
        match self {
            Self::Binary(op, lhs, rhs) => {
                let lhs_value = lhs.eval()?;
                let rhs_value = rhs.eval()?;
                match op {
                    Operator::Add => Ok(lhs_value + rhs_value),
                    Operator::Sub => Ok(lhs_value - rhs_value),
                    Operator::Mul => Ok(lhs_value * rhs_value),
                    Operator::Dev => Ok(lhs_value / rhs_value),
                    Operator::Pow => Ok(i32::pow(lhs_value, rhs_value as u32)),
                    _ => Err(CalculatorError::InternalError("evaluation error"))
                }
            }
            Self::Unary(op, expr) => {
                let expr_value = expr.eval()?;
                match op {
                    Operator::Neg => Ok(-expr_value),
                    _ => Err(CalculatorError::InternalError("evaluation error"))
                }
            }
            Self::Integer(n) => Ok(*n),
        }
    }
}

fn tokenize(s: &str) -> Result<Vec<Token>, CalculatorError> {
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

    fn current_token(&self) -> Result<Token, CalculatorError> {
        if self.current_index >= self.tokens.len() {
            return Err(CalculatorError::InternalError("Index out of range"));
        }
        Ok(self.tokens[self.current_index])
    }
    
    fn primary(&mut self) -> Result<Expression, CalculatorError> {
        let token = self.current_token()?;
        self.consume_token();
        match token {
            Token::Integer(n) => Ok(Expression::Integer(n)),
            Token::OpenParent => {
                let expr = self.expression()?;
                match self.current_token()? {
                    Token::CloseParent => Ok(expr),
                    _ => Err(CalculatorError::SyntaxError("Parenthesis error")),
                }
            }
            Token::Minus => {
                let expr = self.factor()?;
                Ok(Expression::Unary(
                    Operator::Neg,
                    Box::new(expr),
                ))
            }
            _ => {
                Err(CalculatorError::SyntaxError(""))
            }
        }
    }

    fn factor(&mut self) -> Result<Expression, CalculatorError> {
        let expr = self.primary()?;
        let token = self.current_token()?;
        match token {
            Token::Power => {
                self.consume_token();
                let rhs = self.factor()?;
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

    fn term(&mut self) -> Result<Expression, CalculatorError> {
        let mut expr = self.factor()?;

        loop {
            let token = self.current_token()?;
            match token {
                Token::Star => {
                    self.consume_token();
                    let rhs = self.factor()?;
                    expr = Expression::Binary(
                        Operator::Mul,
                        Box::new(expr),
                        Box::new(rhs)
                    );
                }
                Token::Slash => {
                    self.consume_token();
                    let rhs = self.factor()?;
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

    fn expression(&mut self) -> Result<Expression, CalculatorError> {
        let mut expr = self.term()?;
        loop {
            let token = self.current_token()?;
            match token {
                Token::Plus => {
                    self.consume_token();
                    let rhs = self.term()?;
                    expr = Expression::Binary(
                        Operator::Add,
                        Box::new(expr),
                        Box::new(rhs)
                    );
                }
                Token::Minus => {
                    self.consume_token();
                    let rhs = self.term()?;
                    expr = Expression::Binary(
                        Operator::Sub,
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

fn process_calculator_string(input: &str) -> Result<i32, CalculatorError> {
    let tokens: Vec<Token> = tokenize(input)?;
    let mut parser = Parser::new(tokens);
    let expr = parser.expression()?;
    let res = expr.eval()?;
    Ok(res)
}

#[tauri::command]
fn process(input: &str) -> String {
    let result = process_calculator_string(input);
    match result {
        Ok(res) => res.to_string(),
        Err(e) => {
            match e {
                CalculatorError::InternalError(s) => String::from(format!("Internal error: {}", s)),
                CalculatorError::SyntaxError(s) => String::from(format!("Syntax error {}", s)),
            }
        }
    }
}

#[tauri::command]
fn test() -> String {
    let test_cases = [
        ("1+1", 2),
        ("1+2*2", 5),
        ("1+2*3^2", 19),
    ];
    let mut failed = Vec::new();
    for (input_string, expected_output) in test_cases {
        let result = process(input_string).parse::<i32>().unwrap();
        if result != expected_output {
            failed.push((input_string, expected_output, result))
        }
    }
    format!("{:?}", failed)
}