mod nodes;
use crate::lexing::token;
use nodes::Node;
use nodes::ProductionRule;
use token::TokenKind;

struct Exp {}

impl Node for Exp {
    fn get_production_rules() -> Vec<ProductionRule> {
        vec![ProductionRule::Token(TokenKind::IntegerLiteral)]
    }
    
}

struct Statement {

}

impl Node for Statement {
    fn get_production_rules() -> Vec<ProductionRule> {
        vec![
        ProductionRule::Token(TokenKind::ReturnKeyword),
        ProductionRule::Node(Exp {}),
        ProductionRule::Token(TokenKind::Semicolon)
    ]
    }
    
}


fn parse(){

    // defining grammar
    let exp = Node::new(vec![ProductionRule::Token(TokenKind::IntegerLiteral)]);
    let statement = Node::new(vec![
        ProductionRule::Token(TokenKind::ReturnKeyword),
        ProductionRule::Node(exp),
        ProductionRule::Token(TokenKind::Semicolon)
    ]);
    let function = Node::new(vec![
        ProductionRule::Token(TokenKind::Int),
        ProductionRule::Token(TokenKind::Identifier),
        ProductionRule::Token(TokenKind::OpenParen),
        ProductionRule::Token(TokenKind::CloseParen),
        ProductionRule::Token(TokenKind::OpenBrace),
        ProductionRule::Node(statement),
        ProductionRule::Token(TokenKind::CloseBrace)
    ]);
    let program = Node::new(vec![ProductionRule::Node(function)]);


    

}


