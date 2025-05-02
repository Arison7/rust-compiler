use crate::lexing::token::TokenKind;

pub enum ProductionRule {
    Token(TokenKind),
    Node(Node),
}

pub trait Node {
    //production_rules: Vec<ProductionRule>,
    fn get_production_rules() -> Vec<ProductionRule>;
}

