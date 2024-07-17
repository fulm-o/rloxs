mod rloxs_lexer;
mod rloxs_parser;
mod syntax;

use std::{fs::File, io::{Read, Write}, path::Path};

use clap::{Arg, Command};
use rloxs_lexer::Lexer;

fn cli() -> Command {
    Command::new("rloxs")
    .arg(Arg::new("filename")
        .required(false)
        .value_name("FILE")
        .help("Input a .rloxs file.")
    )
}


fn main() {
    let matches: clap::ArgMatches = cli().get_matches();

    if let Some(filepath) = matches.get_one::<String>("filename") {
        let filepath = Path::new(filepath);
        let file = File::open(filepath);
        let mut file = match file {
            Ok(f) => f,
            Err(e) => {
                panic!("{}", e);
            },
        };

        let mut source = String::new();

        if let Err(e) = file.read_to_string(&mut source) {
            panic!("{}", e);
        }

        run(&source);

    }else {
        repl();
    }
}

fn repl() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let line = match line.trim() {
            _ if line.is_empty() => continue,
            line => line,
        };

        run(line);
    }
}

fn run(line: &str) {
    let mut lexer = Lexer::new(line);
    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(e) => {
            panic!("{}", e);
        }
    };

    for t in tokens {
        println!("{}", t);
    }
}