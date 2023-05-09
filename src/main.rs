use anyhow::{Context, Result};
use clap::Parser;
use std::{fmt, path::PathBuf};
mod error;
pub use error::Error;
pub mod parser;
pub mod scanner;
use rustyline::error::ReadlineError;
use rustyline::Editor;
pub use scanner::Scanner;
pub use scanner::{Literal, Token, TokenType};

pub struct Repl {
    history_path: String,
    readline: Editor<()>,
}

impl Repl {
    pub fn new() -> Self {
        let history_path = format!("{}/.seid_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to read history if it exists
        let _ = readline.load_history(&history_path);
        Repl {
            history_path,
            readline,
        }
    }
}

#[derive(Parser)]
#[clap(about = "A runtime for mysterious bytes...")]
struct Args {
    #[clap(parse(from_os_str), help = "Path to Seid file", default_value = "")]
    file_name: PathBuf,
    #[clap(short, long, help = "Arguments to the input file", default_value = "")]
    prog_args: Vec<String>,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file name: {:?}", self.file_name)
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let mut seid = Seid::new(&args);
    seid.start()?;

    Ok(())
}

struct Seid {
    file_name: PathBuf,
    repl: Repl,
    had_error: bool,
    use_prompt: bool,
}

impl Seid {
    fn new(arg: &Args) -> Self {
        let mut run_prompt: bool = false;
        let mut file_name = PathBuf::from("");
        if std::env::args().len() < 2 {
            run_prompt = true;
        } else {
            file_name = arg.file_name.clone();
        }
        Seid {
            file_name: file_name,
            repl: Repl::new(),
            had_error: false,
            use_prompt: run_prompt,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        if self.use_prompt {
            match self.run_prompt() {
                Ok(()) => {}
                Err(e) => self.handle_error(e),
            };
        } else {
            match self.run_file() {
                Ok(()) => {}
                Err(e) => self.handle_error(e),
            }
        }
        Ok(())
    }

    fn handle_error(&mut self, e: Error) {
        match e {
            Error::InputError(_) => {
                eprintln!("{e:?}")
            }
            Error::Repl(_) => {
                eprintln!("{e:?}")
            }
            Error::SyntaxError(_, _, _) => {
                eprintln!("{e:?}");
                self.had_error = true;
            }
            Error::Anyhow(_) => {
                eprintln!("{e:?}")
            }
            Error::ParsingError(_) => {
                eprintln!("{e:?}")
            }
        }
    }

    fn run_prompt(&mut self) -> Result<(), Error> {
        loop {
            match self.repl.readline.readline("(seid) > ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. Ignore it
                    println!("Type \"exit()\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d. Ignore it
                    println!("Type \"exit()\" to exit");
                }
                Err(err) => return Err(Error::Repl(err.to_string())),
                Ok(line) => {
                    if line.eq("exit()") {
                        return Ok(());
                    }
                    match self.run(line) {
                        Ok(()) => (),
                        Err(e) => self.handle_error(e),
                    };
                }
            }
        }
    }

    fn run_file(&self) -> Result<(), Error> {
        let contents = std::fs::read_to_string(&self.file_name).with_context(|| {
            format!("could not read file `{}`", self.file_name.to_str().unwrap())
        })?;
        self.run(contents)?;
        Ok(())
    }

    fn run(&self, source: String) -> Result<(), Error> {
        let mut scanner: Scanner = Scanner::new(source);
        let tokens: &Vec<Token> = scanner.scan_tokens()?;

        for token in tokens {
            println!("{}", token.to_string());
        }
        Ok(())
    }
}
