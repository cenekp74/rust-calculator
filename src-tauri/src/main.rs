#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![process])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Debug)]
enum Token {
    Integer(i32),
    Add,
    Sub,
    Mul,
    Dev,
    Pow,
    Parenthesis(Vec<Token>)
}

enum Function {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Dev(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
}

enum Node {
    Integer(i32),
    Function(Function),
}

fn tokenize(s: &str, parenthesis_level: usize) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut last_c: char = ' ';
    let mut last_number: String = String::new();
    let mut iterations_to_skip: usize = 0;
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            continue;
        }
        if iterations_to_skip > 0 {
            iterations_to_skip -= 1;
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
            tokens.push(Token::Add)
        }
        else if c == '-' {
            tokens.push(Token::Sub)
        }
        else if c == '/' {
            tokens.push(Token::Dev)
        }
        else if c == '*' {
            if last_c == '*' {
                tokens.push(Token::Pow)
            }
            else {
                tokens.push(Token::Mul)
            }
        }
        else if c == '^' {
            tokens.push(Token::Pow)
        }
        else if c == '(' {
            let parenthesis_content = tokenize(&s[i+1..], parenthesis_level+1).unwrap();
            iterations_to_skip = parenthesis_content.len();
            tokens.push(Token::Parenthesis(parenthesis_content));
        }
        else if c == ')' {
            if !(parenthesis_level > 0) {
                return Err(String::from("Parenthesis level error"))
            }
            return Ok(tokens)
        }
        last_c = c;
    }
    tokens.push(Token::Integer(last_number.parse::<i32>().unwrap()));
    Ok(tokens)
}

#[tauri::command]
fn process(input: &str) -> String {
    let tokens: Vec<Token> = tokenize(input, 0).unwrap();
    println!("{:?}", tokens);
    return String::from("---");
}