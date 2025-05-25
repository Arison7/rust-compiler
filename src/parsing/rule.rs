use crate::lexing::token::*;
use crate::parsing::node::*;
use queues::*;

#[derive(Debug)]
pub enum RuleType {
    Token(TokenKind),
    Node(AstNodeKind),
}
#[derive(Debug)]
pub struct Rules {
    pub rule: RuleType,
}
impl Rules {
    pub fn new(rule: RuleType) -> Self {
        Self { rule }
    }

    // Consume a rule to potentially create an AST node
    pub fn consume(&self, tokens: &mut Queue<Token>) -> Result<Option<Node>, String> {
        match &self.rule {
            RuleType::Token(kind) => {
                // Check if the token matched the Token in the queue
                let token = tokens.remove()?;
                //println!("received type: {:?}",t);
                //println!("expected type: {:?}",kind);
                if token.kind() == *kind {
                    //println!("Token consumed");
                    consume_token(token)
                } else {
                    Err(format!(
                        "Incorrect token received:\n  Expected: {:?}\n  Found:    {:?}",
                        kind,
                        token.kind()
                    ))
                }
            }
            // Create new node with correct type and pass the q to it
            RuleType::Node(kind) => {
                let ast_node = kind.get_type();
                Ok(Some(Node::new(ast_node)))
            }
        }
    }
}

// Could be implemented on token, but is part of parsing so i left it here instead
fn consume_token(token: Token) -> Result<Option<Node>, String> {
    match token {
        // Tokens that map to AstNode::String
        Token::StringLiteral(s) | Token::Identifier(s) => Ok(Some(Node::new(AstNode::String(s)))),

        // Tokens that map to AstNode::Int
        Token::IntegerLiteral(i) => Ok(Some(Node::new(AstNode::Int(i)))),

        // Invalid token
        Token::Err(_) => Err("Trying to parse an Err token".to_string()),

        // All others tokens are consumed without creating a node
        _ => Ok(None),
    }
}
