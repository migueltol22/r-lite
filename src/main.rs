use std::io::{self, BufRead, Write};

use r_lite::error::{RLiteError, RLiteResult};
use r_lite::scanner::Scanner;

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

fn process(source: &str) -> Result<(), RLiteError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan()?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn run(source: &str) -> Result<(), RLiteError> {
    let tokens = process(source)?;
    Ok(())
}

fn run_prompt() -> Result<(), RLiteError> {
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

fn main() -> RLiteResult<()> {
    run_prompt()?;
    Ok(())
}
