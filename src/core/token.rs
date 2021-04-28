use super::types::{ Int, Float };
use super::operator::Operator;
use super::symbols::{ TRUE, FALSE };
use crate::errors::PascalineError;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token<'a> {
    Integer(Int),
    Float(Float),
    Bool(bool),
    Operator(&'a Operator),
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

    pub fn new_bool(b: bool) -> Token<'a> {
        Token::Bool(b)
    }

    pub fn new_operator(o: &str) -> Result<Token, PascalineError> {
        Operator::from_symbol(o).map(|op| Token::Operator(op))
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

    pub fn is_number(&self) -> bool {
        match self {
            Token::Integer(_) | Token::Float(_) => true,
            _ => false
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Token::Bool(_) => true,
            _ => false
        }
    }

    pub fn is_operator(&self) -> bool {
        match self {
            Token::Operator(_) => true,
            _ => false
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            &Token::Integer(i) => i == 0,
            &Token::Float(f) => f == 0.0,
            &Token::Bool(b) => !b,
            _ => false
        }
    }

    pub fn as_float(&self) -> Option<Float> {
        match self {
            &Token::Integer(i) => Some(i as Float),
            &Token::Float(f) => Some(f),
            &Token::Bool(b) => Some(if b { 1.0 } else { 0.0 }),
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            &Token::Integer(i) => Some(i != 0),
            &Token::Float(f) => Some(f != 0.0),
            &Token::Bool(b) => Some(b),
            _ => None
        }
    }
}


impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::Integer(i) => write!(formatter, "{}", i),
            &Token::Float(f) => write!(formatter, "{:.6}", f),
            &Token::Bool(b) => write!(formatter, "{}", if b { TRUE } else { FALSE }),
            &Token::Operator(o) => write!(formatter, "{}", o),
            &Token::Ignored => write!(formatter, ""),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::core::Token;
    use crate::core::symbols::ADD;

    #[test]
    fn test_is_ignored() {
        assert!(!Token::new_integer(0).is_ignored());
        assert!(!Token::new_float(0.0).is_ignored());
        assert!(!Token::new_bool(true).is_ignored());
        assert!(!Token::new_operator(ADD).unwrap().is_ignored());
        assert!(Token::new_ignored().is_ignored());
    }

    #[test]
    fn test_is_legit() {
        assert!(Token::new_integer(0).is_legit());
        assert!(Token::new_float(0.0).is_legit());
        assert!(Token::new_bool(true).is_legit());
        assert!(Token::new_operator(ADD).unwrap().is_legit());
        assert!(!Token::new_ignored().is_legit());
    }

    #[test]
    fn test_is_number() {
        assert!(Token::new_integer(0).is_number());
        assert!(Token::new_float(0.0).is_number());
        assert!(!Token::new_bool(true).is_number());
        assert!(!Token::new_operator(ADD).unwrap().is_number());
        assert!(!Token::new_ignored().is_number());
    }

    #[test]
    fn test_is_bool() {
        assert!(!Token::new_integer(0).is_bool());
        assert!(!Token::new_float(0.0).is_bool());
        assert!(Token::new_bool(true).is_bool());
        assert!(!Token::new_operator(ADD).unwrap().is_bool());
        assert!(!Token::new_ignored().is_bool());
    }

    #[test]
    fn test_is_operator() {
        assert!(!Token::new_integer(0).is_operator());
        assert!(!Token::new_float(0.0).is_operator());
        assert!(!Token::new_bool(true).is_operator());
        assert!(Token::new_operator(ADD).unwrap().is_operator());
        assert!(!Token::new_ignored().is_operator());
    }

    #[test]
    fn test_as_float() {
        assert_eq!(Token::new_integer(0).as_float(), Some(0.0));
        assert_eq!(Token::new_float(0.0).as_float(), Some(0.0));
        assert_eq!(Token::new_bool(true).as_float(), Some(1.0));
        assert_eq!(Token::new_bool(false).as_float(), Some(0.0));
        assert_eq!(Token::new_operator(ADD).unwrap().as_float(), None);
        assert_eq!(Token::new_ignored().as_float(), None);
    }

    #[test]
    fn test_as_bool() {
        assert_eq!(Token::new_integer(0).as_bool(), Some(false));
        assert_eq!(Token::new_integer(1).as_bool(), Some(true));
        assert_eq!(Token::new_integer(2).as_bool(), Some(true));
        assert_eq!(Token::new_integer(-1).as_bool(), Some(true));
        assert_eq!(Token::new_float(0.0).as_bool(), Some(false));
        assert_eq!(Token::new_float(1.0).as_bool(), Some(true));
        assert_eq!(Token::new_float(2.0).as_bool(), Some(true));
        assert_eq!(Token::new_float(-1.0).as_bool(), Some(true));
        assert_eq!(Token::new_bool(true).as_bool(), Some(true));
        assert_eq!(Token::new_bool(false).as_bool(), Some(false));
        assert_eq!(Token::new_operator(ADD).unwrap().as_bool(), None);
        assert_eq!(Token::new_ignored().as_bool(), None);
    }
}
