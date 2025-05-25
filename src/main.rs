mod file_processing;
mod lexing;
mod parsing;

use std::fs::File;

fn main() {
    let (f, filename) = file_processing::get_file();

    let mut tokens = lexing::lex(f);

    //println!("{:?}",tokens);
    parsing::parse(&mut tokens).unwrap_or_else(|err| {
        eprintln!("Parsing failed:\n{}", err);
        std::process::exit(1);
    }); //File::create(format!("{}.s",filename)).expect("failed to create file");
}
