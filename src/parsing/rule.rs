use std::option;

use crate::lexing::token::*;
use crate::parsing::node::*;
use queues::*;








#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug)]
pub struct RuleTree {
    pub ruletype: RuleType,
    pub children: Vec<RuleTree>,
}

impl RuleTree {
    pub fn new(ruletype: RuleType) -> Self {
        Self {
            ruletype,
            children: vec![],
        }
    }

    pub fn new_from_path(rule_path: &[Rules]) -> Self {
        if rule_path.is_empty() {
            return RuleTree::new(RuleType::Token(TokenKind::Err));
        }

        let root = RuleTree::new(rule_path[0].rule.clone());

        let mut tree = root;
        tree.add_rule_path(rule_path); // pass the full path including the root

        tree
}

    pub fn add_rule_path(&mut self, rule_path: &[Rules]) {
        if rule_path.is_empty() {
            return;
        }

        let first = &rule_path[0];

        // Only proceed if current node matches the rule
        if self.ruletype != first.rule {
            println!("Mismatched node: expected {:?}, got {:?}", self.ruletype, first.rule);
            return;
        }

        // If this is the last rule in the path, we're done
        if rule_path.len() == 1 {
            return;
        }

        let next_rule = &rule_path[1];

        // Look for a matching child
        if let Some(existing_child) = self
            .children
            .iter_mut()
            .find(|child| child.ruletype == next_rule.rule)
        {
            existing_child.add_rule_path(&rule_path[1..]);
        } else {
            let mut new_child = RuleTree::new(next_rule.rule.clone());
            new_child.add_rule_path(&rule_path[1..]);
            self.children.push(new_child);
        }
    }


    pub fn pretty_print(&self, indent: usize) {
        let indent_str = "  ".repeat(indent);
        println!("{}- {:?}", indent_str, self.ruletype);

        for child in &self.children {
            child.pretty_print(indent + 1);
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
