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
    pub fn parse(mut self: Box<Self>, tokens: &mut Queue<Token>) -> Result<Box<Self>, String> {

        // If there are rules for the current node
        //println!("request rules: {:?}", self.kind);
        if let Some(tree) = AST_RULES.get(&self.kind.get_kind()) {
            // Iterate all the possible branches
            //println!("providing for: {:?}", tree.ruletype);
            self = self.parse_branches(tokens, &tree.children)?;
        }
        Ok(self)
    }
    fn parse_branches(
        mut self: Box<Self>,
        tokens: &mut Queue<Token>,
        branches: & Vec<RuleTree>,
    ) -> Result<Box<Self>, String> {
        for branch in branches {
            // If the was a successful branch break
            if let Ok(res) = branch.ruletype.try_consume(tokens) {
                if let Some(child) = res {
                    // Box the child
                    let mut boxed_child = Box::new(child);
                    // Create child-parent relationship
                    boxed_child.parent = Some(Weak::new());

                    // Parse tokens with the child's rules
                    let boxed_child = boxed_child.parse(tokens)?;

                    // Create parent-child relationship
                    self.children.push(*boxed_child);
                } 
                self = self.parse_branches(tokens, &branch.children)?;
                break;
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
    Function,
    Statement,
    Exp,
    UnOp,
    Int(usize),
    String(String),
    Operator(TokenKind),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum AstNodeKind {
    Program,
    Function,
    Statement,
    Exp,
    UnOp,
    Int,
    String,
    Operator,
}

impl AstNodeKind {
    pub fn get_type(&self) -> AstNode {
        // Only required for higher level nodes
        match self {
            Self::Program => AstNode::Program,
            Self::Function => AstNode::Function,
            Self::Statement => AstNode::Statement,
            Self::Exp => AstNode::Exp,
            Self::UnOp => AstNode::UnOp,
            _ => AstNode::String("err".to_string()),
        }
    }
}

impl AstNode {
    // Construction rules for each node
    // TODO: make those constants
    pub fn get_rules(&self) -> Vec<RuleType> {
        match self {
            AstNode::Exp => vec![RuleType::Token(TokenKind::IntegerLiteral)],
            AstNode::Statement => vec![
                RuleType::Token(TokenKind::ReturnKeyword),
                RuleType::Node(AstNodeKind::Exp),
                RuleType::Token(TokenKind::Semicolon),
            ],
            AstNode::Function => vec![
                RuleType::Token(TokenKind::Int),
                RuleType::Token(TokenKind::Identifier),
                RuleType::Token(TokenKind::OpenParen),
                RuleType::Token(TokenKind::CloseParen),
                RuleType::Token(TokenKind::OpenBrace),
                RuleType::Node(AstNodeKind::Statement),
                RuleType::Token(TokenKind::CloseBrace),
            ],
            AstNode::Program => vec![RuleType::Node(AstNodeKind::Function)],
            _ => vec![],
        }
    }
    pub fn get_kind(&self) -> AstNodeKind {
        match self {
            Self::Program => AstNodeKind::Program,
            Self::Function => AstNodeKind::Function,
            Self::Statement => AstNodeKind::Statement,
            Self::Exp => AstNodeKind::Exp,
            Self::UnOp => AstNodeKind::UnOp,
            Self::Int(_) => AstNodeKind::Int,
            Self::String(_) => AstNodeKind::String,
            Self::Operator(_) => AstNodeKind::Operator,
        }
    }
}
