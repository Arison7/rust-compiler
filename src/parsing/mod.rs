pub mod nodes;
use crate::lexing::token::Token;
use queues::*;
use nodes::Node;
use std::{cell::RefCell, fs::File};
use std::io::prelude::*;

#[derive(Clone)]
struct Exp {
    constant: Option<usize>,
}

impl Exp {
    fn new() -> Self {
        Self { constant: Some(0) }
    }
}

impl<'a> Node<'a> for Exp {
    fn parse(&mut self, tokens: &'a mut Queue<Token>) -> Result<(), &'a str> {
        match tokens.remove() {
            Ok(Token::IntegerLiteral(i)) => {
                self.constant = Some(i);
                Ok(())
            }
            _ => Err("Expected integer literal"),
        }
    }

    fn pretty_print(&self) {
        if let Some(i) = self.constant {
             print!("Int<{}>", i)
        }

    }

    fn generate_assembly(&self,mut f : File) {
        if let Some(con) = self.constant {
            f.write_all(
            format!("movl  ${},%eax\nret\n",con).as_bytes()).expect("failed to write to file");
        }

        
    }
}

struct Statement {
    exp: RefCell<Option<Exp>>,
}

impl Statement {
    fn new() -> Self {
        Self {
            exp: RefCell::new(None),
        }
    }
}

impl<'a> Node<'a> for Statement {
    fn parse(&mut self, tokens: &'a mut Queue<Token>) -> Result<(), &'a str> {
        match tokens.remove() {
            Ok(Token::ReturnKeyword) => {}
            _ => return Err("Expected 'return' keyword"),
        }
    
        let exp = RefCell::new(Some(Exp::new()));
        match exp.borrow_mut().as_mut().unwrap().parse(tokens) {
            Ok(_) => {},
            Err(_) => {return Err("parsing failed")}
        }
        self.exp = exp;

        match tokens.remove() {
            Ok(Token::Semicolon) => {}
            _ => return Err("Expected semicolon"),
        }

        Ok(())
    }

    fn pretty_print(&self) {
        if let Some(ex) = self.exp.borrow().as_ref() {
            ex.pretty_print();
        }
    }
    fn generate_assembly(&self,f : File) {
        if let Some(ex) = self.exp.borrow().as_ref() {
            ex.generate_assembly(f);
        }
    }
}

struct Function {
    name: Option<String>,
    statement: RefCell<Option<Statement>>,
}

impl Function {
    fn new() -> Self {
        Self {
            name: None,
            statement: RefCell::new(None),
        }
    }
}

impl<'a> Node<'a> for Function {
    fn parse(&mut self, tokens: &'a mut Queue<Token>) -> Result<(), &'a str> {
        match tokens.remove() {
            Ok(Token::Int) => {}
            _ => return Err("Expected 'int' keyword"),
        }

        match tokens.remove() {
            Ok(Token::Identifier(n)) => self.name = Some(n),
            _ => return Err("Expected function name"),
        }

        match tokens.remove() {
            Ok(Token::OpenParen) => {}
            _ => return Err("Expected '('"),
        }

        match tokens.remove() {
            Ok(Token::CloseParen) => {}
            _ => return Err("Expected ')'"),
        }

        match tokens.remove() {
            Ok(Token::OpenBrace) => {}
            _ => return Err("Expected '{'"),
        }

        let statement = RefCell::new(Some(Statement::new()));
        //TODO: Change this to actually pass the message 
        match statement.borrow_mut().as_mut().unwrap().parse(tokens) {
            Ok(_) => {},
            Err(_) => {return Err("parsing failed")}
        }
        self.statement = statement;

        match tokens.remove() {
            Ok(Token::CloseBrace) => {}
            _ => return Err("Expected '}'"),
        }

        Ok(())
    }

    fn pretty_print(&self) {
        println!("Function Int {:?}:", self.name.clone().unwrap_or("None".to_string()));
        println!("Body: ");
        if let Some(state) = self.statement.borrow().as_ref() {
            state.pretty_print();
        }
    }
    fn generate_assembly(&self, mut f : File) {
        if let Some(n) = self.name.clone(){
            f.write_all(
            format!(".globl {}\n{}:\n",n,n).as_bytes()).expect("failed to write into file");
        }
        if let Some(state) = self.statement.borrow().as_ref() {
            state.generate_assembly(f);
        }

        
    }
}

pub struct Program {
    function: RefCell<Option<Function>>,
}

impl Program {
    fn new() -> Self {
        Self {
            function: RefCell::new(None),
        }
    }
}

impl<'a> Node<'a> for Program {
    fn parse(&mut self, tokens: &'a mut Queue<Token>) -> Result<(), &'a str> {
        let function = RefCell::new(Some(Function::new()));
        function.borrow_mut().as_mut().unwrap().parse(tokens)?;
        self.function = function;
        Ok(())
    }

    fn pretty_print(&self) {
        if let Some(fun) = self.function.borrow().as_ref() {
            fun.pretty_print();
        }
    }
    fn generate_assembly(&self,f : File) {
        if let Some(fun) = self.function.borrow().as_ref() {
            fun.generate_assembly(f);
        }
    }
}

pub fn parse(tokens: &mut Queue<Token>) -> Result<Program, &str> {
    let program = RefCell::new(Program::new());
    program.borrow_mut().parse(tokens)?;

    // Display AST
    program.borrow().pretty_print();
    Ok(program.into_inner())


    


}
