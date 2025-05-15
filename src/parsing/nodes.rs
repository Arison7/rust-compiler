use queues::Queue;
use crate::lexing::token::Token;
use std::fs::File;

// Trait that all AST-like nodes implement
pub trait Node<'a> {
    fn parse(&mut self,  tokens :&'a mut Queue<Token> ) -> Result<(),&'a str>;
    fn pretty_print(&self);
    fn generate_assembly(&self, f : File);

}

