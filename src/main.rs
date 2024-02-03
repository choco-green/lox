use std::{
    env,
    fs::read_to_string,
    io::{self, stdin, stdout, Write},
    path::Path,
    process::exit,
};

/// The main struct for the Lox programming language
struct Lox {
    had_error: bool,
}

impl Lox {
    /// Creates a new instance of the Lox struct
    fn new() -> Self {
        Self { had_error: false }
    }

    /// Prints the tokens from the source code by splitting the source code by
    /// whitespace
    fn run(&mut self, source: &str) {
        let token = source.split_whitespace().collect::<Vec<&str>>();

        for t in token {
            println!("{}", t);
        }
    }

    /// Creates a new instance of the REPL and runs it
    fn run_prompt(&mut self) -> io::Result<()> {
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

            self.run(&input);

            // Reset the error flag
            self.had_error = false;
        }

        Ok(())
    }

    /// Reads the source code from the file and runs it
    fn run_file(&mut self, file: &Path) -> io::Result<()> {
        let bytes = read_to_string(file)?;
        self.run(&bytes);

        // Indicate an error if there was one
        if self.had_error {
            exit(65);
        }

        Ok(())
    }

    /// The function used to report an error to the user
    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    /// Prints the error message to the user
    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}

fn main() -> io::Result<()> {
    // Collect the command line arguments
    let args = env::args().collect::<Vec<String>>();
    let mut lox = Lox::new();

    // Match the number of arguments
    // Argument of length 1 is omitted because it is the name of the program
    match args.len() {
        2 => {
            if args[1] == "jlox" {
                lox.run_prompt()?;
            }
        }
        3 => lox.run_file(Path::new(&args[1]))?,
        _ => {
            eprintln!("Usage: jlox [script]");
            exit(64);
        }
    }
    Ok(())
}
