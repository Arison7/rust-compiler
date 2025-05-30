use crate::lexing::token::*;
use crate::parsing::rule::*;
use queues::*;
use std::fs::File;
use std::io::Write;
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
        branches: &Vec<RuleTree>,
    ) -> Result<Box<Self>, String> {
        // I don't like that i need to initialize string for this each time 
        let mut err_msg = String::new();
        for branch in branches {
            // If the was a successful branch -> break
            match branch.ruletype.try_consume(tokens) {
                Ok(res) => {
                    err_msg.clear();
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
                Err(e) => {
                    err_msg = e;
                }
            }
        }
        if err_msg.is_empty() {
            Ok(self)
        }else {
            Err(format!("Parsing failed at branch {:?}\n{:?}",self.kind,err_msg))
        }
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
    pub fn generate_assembly(&self, file: &mut File) -> Result<(), String> {
        match self.kind {
            AstNode::Program => {
                for child in &self.children {
                    child.generate_assembly(file)?;
                }
            }
            AstNode::Function => {
                if let AstNode::String(function_name) = &self.children[0].kind {
                    file.write_all(
                        format!(".globl {}\n{}:\n", function_name, function_name).as_bytes(),
                    )
                    .expect("failed to write into file");
                }
                // generate assembly for the statement
                return self.children[1].generate_assembly(file);
            }
            AstNode::Statement => {
                self.children[0].generate_assembly(file)?;
                file.write_all(b"ret\n").expect("failed to write into file");
            }
            AstNode::Exp => {
                // Holds a constant
                // Technically speaking if it's anything other than UnOp we could just call that and put
                // it into the ${} but since the only option for now is int we cannot keep the
                // simplification
                if let AstNode::Int(i) = self.children[0].kind {
                    file.write_all(format!("movl  ${},%eax\n", i).as_bytes())
                        .expect("failed to write into file");
                }
                // Calls the last children first which if it's has UnOp would be Exp so that the
                // value can first be writte into the %eax registery before we perform operations
                // on it
                self.children.last().unwrap().generate_assembly(file)?;

                // generate assemply for all other children, so the UnOp
                for child in &self.children[..(self.children.len() - 1)] {
                    child.generate_assembly(file)?;
                }
            }
            AstNode::UnOp => {
                if let AstNode::Operator(token) = &self.children[0].kind {
                    match token {
                        TokenKind::Negation => {
                            file.write_all(b"neg  %eax\n")
                                .expect("failed to write into file");
                        }
                        TokenKind::LogicalNegation => {
                            file.write_all(b"cmpl   $0, %eax\nmovl   $0, %eax\nsete   %al\n")
                                .expect("failed to write into file");
                        }
                        TokenKind::BitwiseComplement => {
                            file.write_all(b"not  %eax\n")
                                .expect("failed to write into file");
                        }
                        _ => return Err("Incorrect token in Operator field".to_string()),
                    }
                }
            }
            _ => {}
        }

        Ok(())
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
