#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


fn main() {
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![process, test])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn factorial(number : i128) -> Result<f64, CalculatorError> {
    let mut factorial : i128 = 1;
    for i in 1..(number+1) {
        if let Some(n) = factorial.checked_mul(i) {
            factorial = n;
        }
        else {
            return Err(CalculatorError::MathError);
        }
    }
    Ok(factorial as f64)
}

enum CalculatorError {
    SyntaxError(&'static str),
    InternalError(&'static str),
    MathError,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Exclamation,
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
    Fac,
    Neg,
}

#[derive(Debug)]
enum Expression {
    Binary(Operator, Box<Expression>, Box<Expression>),
    Unary(Operator, Box<Expression>),
    Number(f64),
}

impl Expression {
    fn eval(&self) -> Result<f64, CalculatorError> {
        match self {
            Self::Binary(op, lhs, rhs) => {
                let lhs_value = lhs.eval()?;
                let rhs_value = rhs.eval()?;
                match op {
                    Operator::Add => Ok(lhs_value + rhs_value),
                    Operator::Sub => Ok(lhs_value - rhs_value),
                    Operator::Mul => Ok(lhs_value * rhs_value),
                    Operator::Dev => Ok(lhs_value / rhs_value),
                    Operator::Pow => Ok(f64::powf(lhs_value, rhs_value)),
                    _ => Err(CalculatorError::InternalError("evaluation error"))
                }
            }
            Self::Unary(op, expr) => {
                let expr_value = expr.eval()?;
                match op {
                    Operator::Neg => Ok(-expr_value),
                    Operator::Fac => {
                        if expr_value.fract() == 0.0  {
                            factorial(expr_value as i128)
                        }
                        else {
                            Err(CalculatorError::MathError)
                        }
                    },
                    _ => Err(CalculatorError::InternalError("evaluation error"))
                }
            }
            Self::Number(n) => Ok(*n),
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
        if c.is_digit(10) || c == '.' {
            last_number.push(c);
        } 
        else {
            if !last_number.is_empty() {
                let number = last_number.parse::<f64>();
                if let Ok(n) = number {
                    tokens.push(Token::Number(n));
                    last_number.clear()
                }
                else {
                    return Err(CalculatorError::SyntaxError("Wrong float definition"));
                }
                
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
        else if c == '!' {
            tokens.push(Token::Exclamation)
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
        let number = last_number.parse::<f64>();
        if let Ok(n) = number {
            tokens.push(Token::Number(n));
            last_number.clear()
        }
        else {
            return Err(CalculatorError::SyntaxError("Wrong float definition"));
        }
        
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
        let mut result_expression: Expression;
        self.consume_token();
        match token {
            Token::Number(n) => {result_expression = Expression::Number(n)},
            Token::OpenParent => {
                let expr = self.expression()?;
                match self.current_token()? {
                    Token::CloseParent => {result_expression = expr; self.consume_token()},
                    _ => {return Err(CalculatorError::SyntaxError("Parenthesis error"));},
                }
            }
            Token::Minus => {
                let expr = self.factor()?;
                result_expression = Expression::Unary(
                    Operator::Neg,
                    Box::new(expr),
                );
            }
            _ => {
                return Err(CalculatorError::SyntaxError(""));
            }
        }
        loop {
            if let Token::Exclamation = self.current_token()? {
                self.consume_token();
                result_expression = Expression::Unary(Operator::Fac, Box::new(result_expression));
            }
            else {
                break;
            }
        }
        Ok(result_expression)
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

    fn start(&mut self) -> Result<Expression, CalculatorError> {
        let expr = self.expression()?;
        // because of cases when user inputs something that evaluates to a P that is not followed by a binary operator (for exapmle 6(1+1) would evaluate to 6, because everything 
        // after the 6 would be ignored)
        if let Token::End = self.current_token()?  {
            Ok(expr)
        }
        else {
            Err(CalculatorError::SyntaxError(""))
        }
    }
}

fn process_calculator_string(input: &str) -> Result<f64, CalculatorError> {
    let tokens: Vec<Token> = tokenize(input)?;
    let mut parser = Parser::new(tokens);
    let expr = parser.start()?;
    // println!("{:?}", expr);
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
                CalculatorError::MathError => String::from("Math error"),
            }
        }
    }
}

#[tauri::command]
fn test() -> String {
    let test_cases: [(&str, f64); 11] = [
        ("1+1", 2.0),
        ("1+2*2", 5.0),
        ("1+2*3^2", 19.0),
        ("3!", 6.0),
        ("5!", 120.0),
        ("3!!", 720.0),
        ("3*3!+1", 19.0),
        ("(3*3)!-1", 362879.0),
        ("-2*((1+3)*3)", -24.0),
        ("5/2", 2.5),
        ("1.1 + 9.6", 10.7),
    ];
    let mut failed = Vec::new();
    for (input_string, expected_output) in test_cases {
        let result = process(input_string).parse::<f64>().unwrap();
        if result != expected_output {
            failed.push((input_string, expected_output, result))
        }
    }
    format!("{:?}", failed)
}