use crate::lexing::token::TokenKind;

// BUG: fix this lol

pub enum ProductionRule {
    Token(TokenKind),
    Node(Box<dyn Node>),
}


pub trait Node {
    //production_rules: Vec<ProductionRule>,
    fn get_production_rules(&self) -> Vec<ProductionRule>;
    // Returns an iterator for Node 
    fn iter_rules(&self) -> NodeIter<Self> 
    where 
        Self : Sized,
    {
        NodeIter::new(self)
    }
}


// Wrapper Struct to implement iterator on any strcut that implments node
pub struct NodeIter <'a, T: Node> {
    node: &'a T,
    rules: Vec<ProductionRule>,
    index: usize,
}

impl <'a, T:Node> NodeIter<'a,T> {
    pub fn new(node: &'a T) -> Self {
        Self {node , rules : node.get_production_rules(), index : 0}
    }
}

impl<'a, T:Node> Iterator for NodeIter<'a, T > {
    // This will return a copy but since it's just an enum it shouldn't be an issue 
    type Item = TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rules.len() == 0 {
            return None;
        }
        let rule = &self.rules[0];

        match rule {
            ProductionRule::Token(t) => {
                self.rules.pop();
                return Some(*t)
            },
            ProductionRule::Node(n) => {

            }
        }

        None
    }
}

