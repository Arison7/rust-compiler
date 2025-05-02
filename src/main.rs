mod lexing;
mod parsing;

fn main() {
    let tokens = lexing::lex();
    parsing::parse();
}
