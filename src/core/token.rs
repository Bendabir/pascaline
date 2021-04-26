use super::types::{ Int, Float };
use super::operator::Operator;
use crate::errors::PascalineError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(Int),
    Float(Float),
    Operator(Operator),
    Ignored
}

impl Token {
    pub fn new_integer(i: Int) -> Token {
        Token::Integer(i)
    }

    pub fn new_float(f: Float) -> Token {
        if f.fract() == 0.0 {
            Token::Integer(f as Int)
        } else {
            Token::Float(f)
        }
    }

    pub fn new_operator(o: &str) -> Result<Token, PascalineError> {
        Operator::from_symbol(o).map(|op| Token::Operator(op))
    }

    pub fn new_ignored() -> Token {
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

    pub fn is_operator(&self) -> bool {
        match self {
            Token::Operator(_) => true,
            _ => false
        }
    }

    pub fn value(&self) -> Option<Float> {
        match self {
            Token::Integer(i) => Some(i.to_owned() as Float),
            Token::Float(f) => Some(f.to_owned()),
            _ => None
        }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(i) => write!(formatter, "{}", i),
            Token::Float(f) => write!(formatter, "{:.6}", f),
            Token::Operator(o) => write!(formatter, "{}", o),
            Token::Ignored => write!(formatter, ""),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::core::Token;
    use crate::core::symbols::ADD;

    #[test]
    fn test_is_ignored() {
        assert_eq!(Token::new_integer(0).is_ignored(), false);
        assert_eq!(Token::new_float(0.0).is_ignored(), false);
        assert_eq!(Token::new_operator(ADD).unwrap().is_ignored(), false);
        assert_eq!(Token::new_ignored().is_ignored(), true);
    }

    #[test]
    fn test_is_legit() {
        assert_eq!(Token::new_integer(0).is_legit(), true);
        assert_eq!(Token::new_float(0.0).is_legit(), true);
        assert_eq!(Token::new_operator(ADD).unwrap().is_legit(), true);
        assert_eq!(Token::new_ignored().is_legit(), false);
    }

    #[test]
    fn tesnumber() {
        assert_eq!(Token::new_integer(0).is_number(), true);
        assert_eq!(Token::new_float(0.0).is_number(), true);
        assert_eq!(Token::new_operator(ADD).unwrap().is_number(), false);
        assert_eq!(Token::new_ignored().is_number(), false);
    }

    #[test]
    fn test_is_operator() {
        assert_eq!(Token::new_integer(0).is_operator(), false);
        assert_eq!(Token::new_float(0.0).is_operator(), false);
        assert_eq!(Token::new_operator(ADD).unwrap().is_operator(), true);
        assert_eq!(Token::new_ignored().is_operator(), false);
    }

    #[test]
    fn test_value() {
        assert_eq!(Token::new_integer(0).value(), Some(0.0));
        assert_eq!(Token::new_float(0.0).value(), Some(0.0));
        assert_eq!(Token::new_operator(ADD).unwrap().value(), None);
        assert_eq!(Token::new_ignored().value(), None);
    }
}
