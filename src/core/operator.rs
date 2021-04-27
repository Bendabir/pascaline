use super::symbols::{ ADD, SUB, MUL, DIV };
use super::token::Token;
use super::types::Float;
use crate::errors::PascalineError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

// Declaring operators once
const ADD_OPERATOR: Operator = Operator::Add;
const SUB_OPERATOR: Operator = Operator::Sub;
const MUL_OPERATOR: Operator = Operator::Mul;
const DIV_OPERATOR: Operator = Operator::Div;

impl Operator {
    pub fn from_symbol(symbol: &str) -> Result<&Operator, PascalineError> {
        match symbol {
            ADD => Ok(&ADD_OPERATOR),
            SUB => Ok(&SUB_OPERATOR),
            MUL => Ok(&MUL_OPERATOR),
            DIV => Ok(&DIV_OPERATOR),
            s => Err(PascalineError::OperatorSymbolError(s)),
        }
    }

    pub fn arity(&self) -> u8 {
        match self {
            Operator::Add => 2,
            Operator::Sub => 2,
            Operator::Mul => 2,
            Operator::Div => 2
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            Operator::Add => ADD,
            Operator::Sub => SUB,
            Operator::Mul => MUL,
            Operator::Div => DIV
        }
    }

    pub fn operate(&self, operands: &[&Token]) -> Result<Token, PascalineError> {
        // First, check we can unpack enough numbers
        if operands.len() != self.arity() as usize {
            Err(PascalineError::ArityError {
                op: self.symbol(),
                expected:  self.arity(),
                got: operands.len() as u8
            })
        // Then, check we all got numbers
        } else if !Operator::are_numbers(operands) {
            Err(PascalineError::TypeError)
        // Finally, proceed
        } else {
            match self {
                Operator::Add => {
                    let (op1, op2) = Operator::unpack_two(operands);

                    Ok(Token::new_float(op1 + op2))
                },
                Operator::Sub => {
                    let (op1, op2) = Operator::unpack_two(operands);

                    Ok(Token::new_float(op1 - op2))
                },
                Operator::Mul => {
                    let (op1, op2) = Operator::unpack_two(operands);

                    Ok(Token::new_float(op1 * op2))
                },
                Operator::Div => {
                    let (op1, op2) = Operator::unpack_two(operands);

                    if op2 == 0.0 {
                        Err(PascalineError::ZeroDivisionError)
                    } else {
                        Ok(Token::new_float(op1 / op2))
                    }
                }
            }
        }
    }

    fn are_numbers(operands: &[&Token]) -> bool {
        operands.iter().all(|t| t.is_number())
    }

    fn unpack_one(operands: &[&Token]) -> Float {
        // Only used once the size has been checked so it shouldn't be an issue
        operands.first().and_then(|t| t.value()).unwrap()
    }

    fn unpack_two(operands: &[&Token]) -> (Float, Float) {
        // Only used once the size has been checked so it shouldn't be an issue
        (
            operands.get(0).and_then(|t| t.value()).unwrap(),
            operands.get(1).and_then(|t| t.value()).unwrap()
        )
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Operator;
    use crate::core::symbols::ADD;
    use std::ptr;

    #[test]
    fn test_singletons() {
        let op1 = Operator::from_symbol(ADD).unwrap();
        let op2 = Operator::from_symbol(ADD).unwrap();

        assert!(ptr::eq(op1, op2));
    }
}
