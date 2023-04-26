use std::{
    io::{self, BufRead, Write},
};

use r_lite::error::{RLiteError, RLiteResult};

struct Scanner{}

impl Scanner {
    fn new() -> Self {
        Self{}
    }

    fn scan(&self, source: &str) -> Result<Vec<Token>, RLiteError> {
        todo!()
    }
}

#[derive(Debug)]
struct Token{
    _type: TokenType,
}

#[derive(Debug)]
enum TokenType {
    MetaExit,
    StatementInsert,
    StatementSelect
}


fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

fn process(source: &str) -> Result<(), RLiteError> {
    let scanner = Scanner::new();
    let tokens = scanner.scan(source)?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn run(source: &str) -> Result<(), RLiteError> {
    let tokens = process(source)?;
    Ok(())
}

fn run_prompt() -> Result<(), RLiteError>{
    let stdin = io::stdin();
    let mut handler = stdin.lock();

    loop {
        print!("db > ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if handler.read_line(&mut line).is_err() || line.is_empty() {
            break;
        }

        match run(strip_trailing_newline(&line)) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
    }
    Ok(())
}

fn main() -> RLiteResult<()>{
    run_prompt()?;
    Ok(())
}
