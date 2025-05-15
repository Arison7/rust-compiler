mod lexing;
mod parsing;
mod file_processing;

use std::fs::File;
use crate::parsing::nodes::Node;




fn main() {

    let (f,filename) = file_processing::get_file();

    let mut tokens  = lexing::lex(f) ;

    println!("{:?}",tokens);
    let result = parsing::parse(&mut tokens);


    if let Err(res) = result{
        println!("error: {}", res);
    }
    if let Ok(program) = result {
        let output = File::create(format!("{}.s",filename)).expect("failed to create file");
        program.generate_assembly(output);
    }
        

}
