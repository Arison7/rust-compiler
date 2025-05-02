use std::vec;

use regex::Regex;

pub mod token;

use token::Token; 

const TEST: &str = "int main(){
    return 2;
}";

pub fn lex() {
    let mut tokens: Vec<Token> = vec![]; 
    let options = Token::get_options();

    let mut source = &TEST;

    rec_lexing(&mut tokens, source,&options);

    for t in tokens {
        println!("token: {:?}", t);
    }
}

fn rec_lexing(
    tokens: &mut Vec<Token>,
    mut source: & str,
    options: &Vec<&str>,
) {
    if source.len() <= 0 {
        return
    }
    for opt in options {
        let pattern = Token::get_regex(opt);

        let re = Regex::new(pattern).unwrap();

        if let Some(mat) = re.find(source) {
            tokens.push(Token::get_token(opt, mat.as_str()));
            source = & source[mat.len()..];
            rec_lexing( tokens, source, options);
            return;
        }
    }
    let char = &source[..1];
    if char != "\n" && char != r" "{
        tokens.push(Token::get_token("", &source[..1]));
    }
    rec_lexing(tokens, &source[1..], options);

}
