use super::types::{ Int, Float };
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Integer(Int),
    Float(Float),
    Operator(&'a str),
    Ignored
}

impl<'a> Token<'a> {
    pub fn new_integer(i: Int) -> Token<'a> {
        Token::Integer(i)
    }

    pub fn new_float(f: Float) -> Token<'a> {
        if f.fract() == 0.0 {
            Token::Integer(f as Int)
        } else {
            Token::Float(f)
        }
    }

    pub fn new_operator(o: &'a str) -> Token<'a> {
        Token::Operator(o)
    }

    pub fn new_ignored() -> Token<'a> {
        Token::Ignored
    }

    pub fn is_ignored(&self) -> bool {
        match self {
            Token::Ignored => true,
            _ => false
        }
    }

    pub fn is_legit(&self) -> bool {
        !self.is_ignored()
    }
}


impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(i) => write!(formatter, "{}", i),
            Token::Float(f) => write!(formatter, "{}", f),
            Token::Operator(o) => write!(formatter, "{}", o),
            Token::Ignored => write!(formatter, ""),
        }
    }
}
