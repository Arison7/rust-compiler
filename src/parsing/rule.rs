use std::option;

use crate::lexing::token::*;
use crate::parsing::node::*;
use once_cell::sync::Lazy;
use queues::*;
use std::collections::HashMap;

// defining rules

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RuleType {
    Root,
    Token(TokenKind),
    Node(AstNodeKind),
}
impl RuleType {
    /*
    pub fn new(rule: RuleType) -> Self {
        Self { rule }
    }
    */

    // Consume a rule to potentially create an AST node
    pub fn try_consume(&self, tokens: &mut Queue<Token>) -> Result<Option<Node>, String> {
        match &self {
            RuleType::Token(kind) => {
                // Check if the token matched the Token in the queue
                let token = tokens.peek()?;
                //println!("received type: {:?}",t);
                //println!("expected type: {:?}",kind);
                if token.kind() == *kind {
                    //println!("Token consumed");
                    //remove token only if it matches
                    tokens.remove().unwrap();
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
            // root type can be skipped
            RuleType::Root => Ok(None),
        }
    }
}

#[derive(Debug)]
pub struct RuleTree {
    pub ruletype: RuleType,
    pub children: Vec<RuleTree>,
}

impl RuleTree {
    pub fn new_root() -> Self {
        Self {
            ruletype: RuleType::Root,
            children: vec![],
        }
    }

    pub fn new(ruletype: RuleType) -> Self {
        Self {
            ruletype,
            children: vec![],
        }
    }

    pub fn pretty_print(&self, indent: usize) {
        let indent_str = "  ".repeat(indent);
        println!("{}- {:?}", indent_str, self.ruletype);

        for child in &self.children {
            child.pretty_print(indent + 1);
        }
    }

    pub fn insert_path(&mut self, rule_path: &[RuleType]) {
        if rule_path.is_empty() {
            return;
        }
        // Get the first element in the rule_path
        let first = &rule_path[0];

        // Check if there is already existing child with that path
        if let Some(existing_child) = self
            .children
            .iter_mut()
            .find(|child| child.ruletype == *first)
        {
            // If so continue inserting from there
            existing_child.insert_path(&rule_path[1..]);
        } else {
            let mut new_child = RuleTree::new(first.clone());
            new_child.insert_path(&rule_path[1..]);
            self.children.push(new_child);
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

        // UnOp tokens
        Token::LogicalNegation | Token::Negation | Token::BitwiseComplement => {
            Ok(Some(Node::new(AstNode::Operator(token.kind()))))
        }

        // Invalid token
        Token::Err(_) => Err("Trying to parse an Err token".to_string()),

        // All others tokens are consumed without creating a node
        _ => Ok(None),
    }
}

pub static AST_RULES: Lazy<HashMap<AstNodeKind, RuleTree>> = Lazy::new(|| {
    use AstNodeKind::*;
    use RuleType::*;
    use TokenKind::*;

    let mut rule_map: HashMap<AstNodeKind, RuleTree> = HashMap::new();

    println!("called");

    // unOp: three unary operator possibilities
    let unOp: [Vec<RuleType>; 3] = [
        vec![Token(Negation)],
        vec![Token(BitwiseComplement)],
        vec![Token(LogicalNegation)],
    ];

    // exp: two alternatives
    let exp: [Vec<RuleType>; 2] = [vec![Token(IntegerLiteral)], vec![Node(UnOp), Node(Exp)]];

    // statement: one alternative
    let statement: [Vec<RuleType>; 1] = [vec![Token(ReturnKeyword), Node(Exp), Token(Semicolon)]];

    // function: one alternative
    let function: [Vec<RuleType>; 1] = [vec![
        Token(TokenKind::Int),
        Token(Identifier),
        Token(OpenParen),
        Token(CloseParen),
        Token(OpenBrace),
        Node(Statement),
        Token(CloseBrace),
    ]];

    // program: one alternative
    let program: [Vec<RuleType>; 1] = [vec![Node(Function)]];

    // Combine all into one list of tuples: (AstNodeKind, &[Vec<RuleType>])
    let all_rules: &[(AstNodeKind, &[Vec<RuleType>])] = &[
        (UnOp, &unOp),
        (Exp, &exp),
        (Statement, &statement),
        (Function, &function),
        (Program, &program),
    ];

    // Iterate over all paths and add their rules in tries
    for (kind, paths) in all_rules {
        // First path
        let mut root = RuleTree::new_root();
        

        // All the remainings paths
        for path in *paths {
            root.insert_path(path);
        }

        rule_map.insert(kind.clone(),root);
    }

    rule_map
});
