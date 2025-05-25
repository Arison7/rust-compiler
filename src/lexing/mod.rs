use queues::*;

use regex::Regex;
use token::Token; 
use std::{fs::File, io::Read};

pub mod token;


pub fn lex(mut source : File )  -> Queue<Token>{
    let mut tokens: Queue<Token> = queue![]; 
    let options = Token::get_options();

    let mut content : String = String::new();

    source.read_to_string(&mut content).expect("Couldn't read file");


    rec_lexing(&mut tokens,&content ,&options);

    tokens
}

fn rec_lexing(
    tokens: &mut Queue<Token>,
    mut source: & str,
    options: &Vec<&str>,
) {
    // Return if the source is empty
    if source.is_empty() {
        return
    }
    // For each token option
    for opt in options {
        // Get its regex
        let pattern = Token::get_regex(opt);
        
        // Create new regex instance
        let re = Regex::new(pattern).unwrap();

        // Try to match to the begining of the source (each regex starts with ^)
        if let Some(mat) = re.find(source) {
            tokens.add(Token::get_token(opt, mat.as_str())).unwrap();
            // Remove the matched part of the source
            source = & source[mat.len()..];
            rec_lexing( tokens, source, options);
            return;
        }
    }
    let char = &source[..1];
    // Ignoring white spaces, end of the line characters and carriage return since they are irrelevant in C
    //println!("{:?}",char);
    if char != "\n" && char != r" " && char != "\r"{
        tokens.add(Token::get_token("", &source[..1])).unwrap();
    }
    rec_lexing(tokens, &source[1..], options);

}
