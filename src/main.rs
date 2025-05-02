mod lexing;
mod parsing;

fn main() {
    lexing::lex();
    parsing::parse();
}
