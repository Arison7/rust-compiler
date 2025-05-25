use crate::lexing::token::*;
use crate::parsing::rule::*;
use queues::*;
use std::rc::Weak;

#[derive(Debug)]
pub struct Node {
    pub kind: AstNode,
    pub children: Vec<Node>,
    pub parent: Option<Weak<Node>>,
}

impl Node {
    pub fn parse(mut self: Box<Self>, tokens: &mut Queue<Token>) -> Result<Box<Self>,String> {
        //println!("request rules: {:?}", self.kind);

        // Iterate over construction rules of a node
        for rule in self.kind.get_rules() {
            //println!("rule: {:?}",rule);

            // If rule up on consumetion creates a node
            if let Some(child) = rule.consume(tokens)? {
                //println!("child from the rule {:?}",child);

                // Box the child
                let mut boxed_child = Box::new(child);
                // Create child-parent relationship
                boxed_child.parent = Some(Weak::new());

                // Parse tokens with the child's rules
                let boxed_child = boxed_child.parse(tokens)?;

                // Create parent-child relationship
                self.children.push(*boxed_child);
            } else {
                // If no child was node was created from the rule
                // nothing needs to be done
                //println!("Nothing from the rule {:?}", tokens)
            }
        }
        Ok(self)
    }
    // Constructor
    pub fn new(kind: AstNode) -> Self {
        Self {
            kind,
            children: vec![],
            parent: None,
        }
    }

    pub fn pretty_print(&self, indent: usize) {
        let indent_str = "  ".repeat(indent);
        println!("{}{:?}", indent_str, self.kind);

        for child in &self.children {
            child.pretty_print(indent + 1);
        }
    }
}

#[derive(Debug)]
pub enum AstNode {
    Program,
    Statement,
    Exp,
    Int(usize),
    String(String),
    BinOp(String),
    Function,
}

#[derive(Debug)]
pub enum AstNodeKind {
    Program,
    Function,
    Statement,
    Exp,
    Int,
    String,
    BinOp,
}

impl AstNodeKind {
    pub fn get_type(&self) -> AstNode {
        // Only required for higher level nodes
        match self {
            Self::Program => AstNode::Program,
            Self::Function => AstNode::Function,
            Self::Statement => AstNode::Statement,
            Self::Exp => AstNode::Exp,
            _ => AstNode::String("err".to_string()),
        }
    }
}

impl AstNode {
    // Construction rules for each node
    // TODO: make those constants
    pub fn get_rules(&self) -> Vec<Rules> {
        match self {
            AstNode::Exp => vec![Rules::new(RuleType::Token(TokenKind::IntegerLiteral))],
            AstNode::Statement => vec![
                Rules::new(RuleType::Token(TokenKind::ReturnKeyword)),
                Rules::new(RuleType::Node(AstNodeKind::Exp)),
                Rules::new(RuleType::Token(TokenKind::Semicolon)),
            ],
            AstNode::Function => vec![
                Rules::new(RuleType::Token(TokenKind::Int)),
                Rules::new(RuleType::Token(TokenKind::Identifier)),
                Rules::new(RuleType::Token(TokenKind::OpenParen)),
                Rules::new(RuleType::Token(TokenKind::CloseParen)),
                Rules::new(RuleType::Token(TokenKind::OpenBrace)),
                Rules::new(RuleType::Node(AstNodeKind::Statement)),
                Rules::new(RuleType::Token(TokenKind::CloseBrace)),
            ],
            AstNode::Program => vec![Rules::new(RuleType::Node(AstNodeKind::Function))],
            _ => vec![],
        }
    }
}
