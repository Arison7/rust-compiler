pub mod node;
mod rule;

use crate::lexing::token::Token;
use node::*;
use queues::*;



pub fn parse(tokens : &mut Queue<Token>) -> Result<Node,String>{
    
    // Create a program node
    let program = Box::new(Node::new(AstNode::Program));
    // Parse tokens into an AST tree with program as a root node
    let program = program.parse(tokens)?;

    // println!("Program: {:?}", *program);
    program.pretty_print(0);

    Ok(*program)


}
