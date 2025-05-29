mod file_processing;
mod lexing;
mod parsing;

use std::fs::File;

// temporary imports for debugging
use lexing::token::*;
use parsing::node::*;
use parsing::rule::*;

fn main() {
    //let (f, filename) = file_processing::get_file();

    //let mut tokens = lexing::lex(f);
    let temp = vec![
        Rules::new(RuleType::Token(TokenKind::Int)),
        Rules::new(RuleType::Token(TokenKind::Identifier)),
        Rules::new(RuleType::Token(TokenKind::OpenParen)),
        Rules::new(RuleType::Token(TokenKind::CloseParen)),
        Rules::new(RuleType::Token(TokenKind::OpenBrace)),
        Rules::new(RuleType::Node(AstNodeKind::Statement)),
        Rules::new(RuleType::Token(TokenKind::CloseBrace)),
    ];
    let temp2 = vec![
        Rules::new(RuleType::Token(TokenKind::Int)),
        Rules::new(RuleType::Token(TokenKind::Identifier)),
        Rules::new(RuleType::Token(TokenKind::OpenParen)),
        Rules::new(RuleType::Token(TokenKind::CloseBrace)),
    ];

    let mut root = RuleTree::new_from_path(&temp);

    root.pretty_print(0);

    root.add_rule_path(&temp2);

    root.pretty_print(0);

    //println!("{:?}",tokens);
    //*
    //parsing::parse(&mut tokens).unwrap_or_else(|err| {
    //    eprintln!("Parsing failed:\n{}", err);
    //    std::process::exit(1);
    //}); //File::create(format!("{}.s",filename)).expect("failed to create file");
    //
}
