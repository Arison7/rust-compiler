use crate::lexing::token::TokenKind;

// Recursive rule type: either a token or another node
pub enum ProductionRule {
    Token(TokenKind),
    Node(Box<dyn Node>),
}

// Trait that all AST-like nodes implement
pub trait Node {
    fn get_production_rules(&self) -> Vec<ProductionRule>;

    fn iter_rules(&self) -> NodeIter {
        NodeIter::new(self.get_production_rules())
    }
}

// Iterator that walks through nested Node structures
pub struct NodeIter {
    stack: Vec<ProductionRule>,
}

impl NodeIter {
    pub fn new(rules : Vec<ProductionRule>) -> Self {
        Self {
            stack: rules,
        }
    }
}

impl Iterator for NodeIter {
    type Item = TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(rule) = self.stack.pop() {
            match rule {
                ProductionRule::Token(t) => return Some(t),
                ProductionRule::Node(n) => {
                    let mut inner_iter = n.iter_rules().collect::<Vec<_>>();
                    inner_iter.reverse(); // maintain order for the stack
                    for token in inner_iter {
                        self.stack.push(ProductionRule::Token(token));
                    }
                }
            }
        }
        None
    }
}

