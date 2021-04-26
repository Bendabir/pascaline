use super::symbols::{ ADD, SUB, MUL, DIV };
use crate::errors::PascalineError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

impl Operator {
    pub fn from_symbol(symbol: &str) -> Result<Operator, PascalineError> {
        match symbol {
            ADD => Ok(Operator::Add),
            SUB => Ok(Operator::Sub),
            MUL => Ok(Operator::Mul),
            DIV => Ok(Operator::Div),
            s => Err(PascalineError::OperatorSymbolError(s.to_owned())),
        }
    }

    pub fn arity(&self) -> u8 {
        match self {
            Operator::Add => 2,
            Operator::Sub => 2,
            Operator::Mul => 2,
            Operator::Div => 2,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Operator::Add => ADD,
                Operator::Sub => SUB,
                Operator::Mul => MUL,
                Operator::Div => DIV,
            }
        )
    }
}
