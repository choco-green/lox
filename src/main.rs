mod scanner;
mod token;

use std::{
    env,
    fs::read_to_string,
    io::{self, stdin, stdout, Write},
    path::Path,
    process::exit,
};

use scanner::Scanner;

fn main() -> io::Result<()> {
    // Collect the command line arguments
    let args = env::args().collect::<Vec<String>>();

    // Match the number of arguments
    // Argument of length 1 is omitted because it is the name of the program
    match args.len() {
        2 => {
            if args[1] == "jlox" {
                run_prompt()?;
            }
        }
        3 => run_file(Path::new(&args[1]))?,
        _ => {
            eprintln!("Usage: jlox [script]");
            exit(64);
        }
    }
    Ok(())
}

/// Prints the tokens from the source code by splitting the source code by
/// whitespace
fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{token:?}");
    }
}

/// Creates a new instance of the REPL and runs it
fn run_prompt() -> io::Result<()> {
    loop {
        // Prompts user for the next command
        print!("> ");
        stdout().flush()?;

        // Reads the input from the user
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        // If the input is empty, break the loop
        if input.trim().is_empty() {
            break;
        }

        run(&input);
    }

    Ok(())
}

/// Reads the source code from the file and runs it
fn run_file(file: &Path) -> io::Result<()> {
    let bytes = read_to_string(file)?;
    run(&bytes);

    Ok(())
}
