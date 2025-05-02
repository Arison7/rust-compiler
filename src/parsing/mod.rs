mod nodes;
use crate::lexing::token;
use nodes::Node;
use nodes::ProductionRule;
use token::TokenKind;

struct Exp {}

impl Node for Exp {
    fn get_production_rules(&self) -> Vec<ProductionRule> {
        vec![ProductionRule::Token(TokenKind::IntegerLiteral)]
    }
}

struct Statement {}

impl Node for Statement {
    fn get_production_rules(&self) -> Vec<ProductionRule> {
        vec![
            ProductionRule::Token(TokenKind::ReturnKeyword),
            ProductionRule::Node(Box::new(Exp {})),
            ProductionRule::Token(TokenKind::Semicolon),
        ]
    }
}

struct Function {}

impl Node for Function {
    fn get_production_rules(&self) -> Vec<ProductionRule> {
        vec![
            ProductionRule::Token(TokenKind::Int),
            ProductionRule::Token(TokenKind::Identifier),
            ProductionRule::Token(TokenKind::OpenParen),
            ProductionRule::Token(TokenKind::CloseParen),
            ProductionRule::Token(TokenKind::OpenBrace),
            ProductionRule::Node(Box::new(Statement {})),
            ProductionRule::Token(TokenKind::CloseBrace),
        ]
    }
}

struct Program {
    
}

impl Node for Program {
    fn get_production_rules(&self) -> Vec<ProductionRule> {
        vec![ProductionRule::Node(Box::new(Function {}))]
    }
    
}

fn parse() {


}
