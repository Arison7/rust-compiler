mod nodes;
use crate::lexing::token;
use crate::lexing::token::Token;
use nodes::Node;
use token::TokenKind;
use std::cell::RefCell;

#[derive(Clone)]
struct Exp {
    constant : usize
}

impl Exp {
    fn new() -> Self {
        Self {
            constant : 0
        }
    }
}
impl Node for Exp {

    fn parse(&mut self, tokens: &mut Vec<Token>) -> Option<()>
    {
        if let Some(t) = tokens.pop() { 
            match t {
                Token::IntegerLiteral(i) => { self.constant = i}
                _ => {return None}
            }
        }else{
            return None
        }
        Some(())
    }

}

struct Statement {

    exp : RefCell<Option<Exp>>

}

impl Statement {
    fn new () -> Self {
        Self{ 
            exp : RefCell::new(None)
        }

    }
    
}

impl Node for Statement {
    fn parse(&mut self,  tokens :&mut Vec<Token> ) -> Option<()>{
        if tokens.pop()? != Token::ReturnKeyword{
            return None;
        } 

        let exp = RefCell::new(Some(Exp::new()));
        // I needed a mutable referance, not sure if that's the best way of doing it 
        // acutally doubt so
        exp.borrow_mut().as_mut().unwrap().parse(tokens);
        self.exp = exp;

        if tokens.pop()? != Token::Semicolon {
            return None;
        } 

        Some(())
        
    }
    
}

struct Function {
    name : Option<String>, 
    statement : RefCell<Option<Statement>>,
}
impl Function {
    fn new() -> Self {
        Self{
            name : None,
            statement : RefCell::new(None)
        }

    }
    
}

impl Node for Function {
    fn parse(&mut self,  tokens :&mut Vec<Token> ) -> Option<()> {
        if tokens.pop()? != Token::Int{
            return None;
        } 

        let name = tokens.pop()?;
        match name {
            Token::Identifier(n) => {self.name = Some(n)}
            _ => {return None}
        }

        if tokens.pop()? != Token::OpenParen{
            return None;
        } 

        if tokens.pop()? != Token::CloseParen{
            return None;
        } 
        
        if tokens.pop()? != Token::OpenBrace{
            return None;
        } 

        let statement = RefCell::new(Some(Statement::new()));

        statement.borrow_mut().as_mut().unwrap().parse(tokens);

        self.statement = statement;
        

        if tokens.pop()? != Token::CloseBrace{
            return None;
        } 




        Some(())
    }

}

struct Program {
    function : RefCell<Option<Function>>
}

impl Node for Program {
    fn parse(&mut self,  tokens :&mut Vec<Token> ) -> Option<()> {

        let function = RefCell::new(Some(Function::new()));

        function.borrow_mut().as_mut().unwrap().parse(tokens);

        self.function = function;

        Some(())
    }


}

pub fn parse(tokens : Vec<Token>) {

}




