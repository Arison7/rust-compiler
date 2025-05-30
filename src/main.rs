mod file_processing;
mod lexing;
mod parsing;

use std::fs::File;


fn main() {
    let (f, filename) = file_processing::get_file();
    

    let mut tokens = lexing::lex(f);


    
    //println!("{:?}",tokens);
    let program = parsing::parse(&mut tokens).unwrap_or_else(|err| {
        eprintln!("Parsing failed:\n{}", err);
        std::process::exit(1);
    });

    let mut new_file = File::create(format!("{}.s",filename)).expect("failed to create file");

    program.generate_assembly(&mut new_file).unwrap_or_else(|err | {
        eprintln!("Parsing failed:\n{}", err);
        std::process::exit(1);
    });

}
