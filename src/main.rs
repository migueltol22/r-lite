use std::{
    io::{self, BufRead, Write},
    process,
};

use r_lite::error::RLiteError;

enum RLiteCommands {
    MetaCommand,
    Statement,
}

enum StatementType {
    StatementInsert,
    StatementSelect,
}

struct Statement {
    _type: StatementType,
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

fn process_meta_command(source: &str) -> Result<(), RLiteError> {
    match source {
        ".exit" => process::exit(0),
        _ => RLiteError::MetaCommmandError("Unrecognized command {source}".to_string()),
    };
    Ok(())
}

fn process_statement(source: &str) -> Result<Statement, RLiteError> {
    match source {
        "insert" => Ok(Statement {
            _type: StatementType::StatementInsert,
        }),
        "select" => Ok(Statement {
            _type: StatementType::StatementSelect,
        }),
        _ => Err(RLiteError::StatementError(
            "Unrecognized keyword at start of {source}".to_string(),
        )),
    }
}

fn execute_statement(statement: Statement) -> Result<(), RLiteError> {
    todo!()
}

fn run(source: &str) -> Result<(), RLiteError> {
    match source.chars().nth(0) {
        Some('.') => process_meta_command(source)?,
        _ => {
            let statement = process_statement(source)?;
            execute_statement(statement)?;
        }
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut handler = stdin.lock();

    loop {
        print!("db > ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if handler.read_line(&mut line).is_err() || line.is_empty() {
            return;
        }

        match run(strip_trailing_newline(&line)) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
    }
}

fn main() {
    run_prompt();
}
