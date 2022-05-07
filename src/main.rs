use clap::Parser;
use std::{
    fmt,
    path::PathBuf,
};
use anyhow::{Context, Result};
mod error;
pub use error::Error;
mod scanner;
pub use scanner::Scanner;
mod token;
pub use token::Token;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Repl {
    history_path: String,
    readline: Editor<()>,
}

impl Repl {
    pub fn new() -> Repl {
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
    #[clap(
        parse(from_os_str),
        help = "Path to Seid file"
    )]
    file_name: PathBuf,
    #[clap(
        short,
        long,
        help = "Arguments to the input file",
        default_value = vec![]
    )]
    prog_args: Vec<String>,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file name: {:?}", self.file_name)
    }
}

fn main() -> Result<(), anyhow::Error>{
    let args = Args::parse();
    
    Ok(())
}

struct Seid {
    file_name:  PathBuf,
    repl:       Repl,
    had_error:  bool,
    use_prompt: bool,
}

impl Seid{
    fn new(arg: &Args) -> Self {
        let mut run_prompt: bool = false;
        let mut file_name = PathBuf::from("");
        if std::env::args().len() < 2 {
            run_prompt = true;
        }
        else {
            file_name = arg.file_name;
        }
        Seid {
            file_name: file_name,
            repl:  Repl::new(),
            had_error: false,
            use_prompt: run_prompt,
        }
    }

    fn start(&self) -> Result<(), Error> {
        if self.use_prompt {
            match self.run_prompt() {
                Ok(()) => {},
                Err(e) => self.handle_error(e),
            };
        }
        else {
            match self.run_file() {
                Ok(()) => {},
                Err(e) => self.handle_error(e),
            }
        }
        Ok(())
    }

    fn handle_error(&self, e: Error) {
        match e {
            Error::InputError(s) => {
                eprintln!("{:?}", e)
            },
            Error::Repl(s) => {
                eprintln!("{:?}", e)
            },
            Error::SyntaxError(s1, s2, s3) => {
                eprintln!("{:?}", e);
                self.had_error = true;
            }
            Error::Anyhow(s) => {
                eprintln!("{:?}", e)
            },
        }
    }

    fn run_prompt(&self) -> Result<(), Error> {
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
                Err(err) => {
                    return Err(Error::Repl(err.to_string()))
                }
                Ok(line) => {
                    if line.eq("exit()") {
                        return Ok(());
                    }
                    self.run(&line)?;
                }
            }
        } 
    }

    fn run_file(&self) -> Result<(), Error> {
        let contents = std::fs::read_to_string(self.file_name)
            .with_context(|| format!("could not read file `{}`", self.file_name.to_str().unwrap()))?;
        self.run(&contents)?;
        Ok(())
    }

    fn run(&self, source: &str) -> Result<(), Error> {
        let mut scanner: Scanner = Scanner::new(source);
        let tokens:  Vec<Token> = scanner.scan_tokens()?;

        for token in tokens {
            println!("{:?}", token);
        }
        Ok(())
    }
}