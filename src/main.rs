use std::io;
use std::collections::{HashMap, HashSet, VecDeque};

/// Represents a logical expression
#[derive(Debug, Clone)]
enum Expr {
    Var(char),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Implies(Box<Expr>, Box<Expr>),
    Biconditional(Box<Expr>, Box<Expr>),
}

/// Operator precedence
fn precedence(op: &str) -> i32 {
    match op {
        "!" => 4, // Highest precedence
        "&" => 3,
        "|" => 2,
        "->" => 1,
        "<->" => 0, // Lowest precedence
        _ => -1,
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars:Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' => {}
            'A'..='Z' | 'a'..='z' => tokens.push(chars[i].to_string()),
            '!' | '&' | '|' | '(' | ')' => tokens.push(chars[i].to_string()),
            '-' if i+1 < chars.len() && chars[i+1] == '>' => { tokens.push("->".to_string()); i+=1 }
            '<' if i+2 < chars.len() && chars[i+1] == '-' && chars[i+2] == '>' => { tokens.push("<->".to_string()); i+=2 }
            _ => panic!("Invalid character {}", chars[i])
        }
        i += 1;
    }

    tokens
}

fn infix_to_postfix(input: &str) -> VecDeque<String> {
    let mut output:VecDeque<String> = VecDeque::new();
    let mut operators:Vec<String> = Vec::new();

    let tokens = tokenize(input);

    for token in tokens {
        match token.as_str() {
            " " => {}, // Ignore spaces
            _ if token.chars().all(|c| c.is_alphabetic()) => output.push_back(token), // Variables
            "!" | "&" | "|" | "->" | "<->" => {
                while let Some(top) = operators.last() {
                    if precedence(top) < precedence(&token) {
                        break;
                    }
                    output.push_back(operators.pop().unwrap());
                }
                operators.push(token);
            }
            "(" => operators.push(token),
            ")" => {
                while let Some(top) = operators.pop() {
                    if top == "(" {
                        break;
                    }
                    output.push_back(top);
                }
            }
            _ => panic!("Invalid token: {}", token),
        }
    }

    while let Some(op) = operators.pop() {
        output.push_back(op);
    }

    output

}

// fn postfix_to_ast(postfix: VecDeque<String>) -> Expr {

// }



fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();
    stdin.read_line(input);
    input.pop();
    input.pop();

    let teste = infix_to_postfix(input);
    for test in teste {
        println!("{}", test);
    }
}
