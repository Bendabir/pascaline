use super::symbols::*;
use super::token::Token;
use super::types::Float;
use crate::errors::PascalineError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    EucDiv,
    Mod,
    Pow,
    Neg,
    Sin,
    Cos,
    Tan,
    ArcSin,
    ArcCos,
    ArcTan,
    Sqrt,
    Exp,
    Ln,
    // Num,
    // Den,
    // COMPLEX OPERATORS
    // Complex,
    // Re,
    // Im,
    // Arg,
    // Norm,
    // LOGICAL OPERATORS
    Eq,
    Neq,
    Le,
    Lt,
    Ge,
    Gt,
    And,
    Or,
    Not,
    // STACK OPERATORS
    Dup,
    Drop,
    Swap,
    LastOp,
    LastArgs,
    Undo,
    Redo,
    Clear
}

// Declaring operators once
const ADD_OPERATOR: Operator = Operator::Add;
const SUB_OPERATOR: Operator = Operator::Sub;
const MUL_OPERATOR: Operator = Operator::Mul;
const DIV_OPERATOR: Operator = Operator::Div;
const EUC_DIV_OPERATOR: Operator = Operator::EucDiv;
const MOD_OPERATOR: Operator = Operator::Mod;
const POW_OPERATOR: Operator = Operator::Pow;
const NEG_OPERATOR: Operator = Operator::Neg;
const SIN_OPERATOR: Operator = Operator::Sin;
const COS_OPERATOR: Operator = Operator::Cos;
const TAN_OPERATOR: Operator = Operator::Tan;
const ARCSIN_OPERATOR: Operator = Operator::ArcSin;
const ARCCOS_OPERATOR: Operator = Operator::ArcCos;
const ARCTAN_OPERATOR: Operator = Operator::ArcTan;
const SQRT_OPERATOR: Operator = Operator::Sqrt;
const EXP_OPERATOR: Operator = Operator::Exp;
const LN_OPERATOR: Operator = Operator::Ln;
// const NUM_OPERATOR: Operator = Operator::Num;
// const DEN_OPERATOR: Operator = Operator::Den;
// const COMPLEX_OPERATOR: Operator = Operator::Complex;
// const RE_OPERATOR: Operator = Operator::Re;
// const IM_OPERATOR: Operator = Operator::Im;
// const ARG_OPERATOR: Operator = Operator::Arg;
// const NORM_OPERATOR: Operator = Operator::Norm;
const EQ_OPERATOR: Operator = Operator::Eq;
const NEQ_OPERATOR: Operator = Operator::Neq;
const LE_OPERATOR: Operator = Operator::Le;
const LT_OPERATOR: Operator = Operator::Lt;
const GE_OPERATOR: Operator = Operator::Ge;
const GT_OPERATOR: Operator = Operator::Gt;
const AND_OPERATOR: Operator = Operator::And;
const OR_OPERATOR: Operator = Operator::Or;
const NOT_OPERATOR: Operator = Operator::Not;
const DUP_OPERATOR: Operator = Operator::Dup;
const DROP_OPERATOR: Operator = Operator::Drop;
const SWAP_OPERATOR: Operator = Operator::Swap;
const LASTOP_OPERATOR: Operator = Operator::LastOp;
const LASTARGS_OPERATOR: Operator = Operator::LastArgs;
const UNDO_OPERATOR: Operator = Operator::Undo;
const REDO_OPERATOR: Operator = Operator::Redo;
const CLEAR_OPERATOR: Operator = Operator::Clear;

impl Operator {
    pub fn from_symbol(symbol: &str) -> Result<&Operator, PascalineError> {
        match symbol {
            ADD => Ok(&ADD_OPERATOR),
            SUB => Ok(&SUB_OPERATOR),
            MUL => Ok(&MUL_OPERATOR),
            DIV => Ok(&DIV_OPERATOR),
            EUC_DIV => Ok(&EUC_DIV_OPERATOR),
            MOD => Ok(&MOD_OPERATOR),
            POW => Ok(&POW_OPERATOR),
            NEG => Ok(&NEG_OPERATOR),
            SIN => Ok(&SIN_OPERATOR),
            COS => Ok(&COS_OPERATOR),
            TAN => Ok(&TAN_OPERATOR),
            ARCSIN => Ok(&ARCSIN_OPERATOR),
            ARCCOS => Ok(&ARCCOS_OPERATOR),
            ARCTAN => Ok(&ARCTAN_OPERATOR),
            SQRT => Ok(&SQRT_OPERATOR),
            EXP => Ok(&EXP_OPERATOR),
            LN => Ok(&LN_OPERATOR),
            // NUM => Ok(&NUM_OPERATOR),
            // DEN => Ok(&DEN_OPERATOR),
            // COMPLEX => Ok(&COMPLEX_OPERATOR),
            // RE => Ok(&RE_OPERATOR),
            // IM => Ok(&IM_OPERATOR),
            // ARG => Ok(&ARG_OPERATOR),
            // NORM => Ok(&NORM_OPERATOR),
            EQ => Ok(&EQ_OPERATOR),
            NEQ => Ok(&NEQ_OPERATOR),
            LE => Ok(&LE_OPERATOR),
            LT => Ok(&LT_OPERATOR),
            GE => Ok(&GE_OPERATOR),
            GT => Ok(&GT_OPERATOR),
            AND => Ok(&AND_OPERATOR),
            OR => Ok(&OR_OPERATOR),
            NOT => Ok(&NOT_OPERATOR),
            DUP => Ok(&DUP_OPERATOR),
            DROP => Ok(&DROP_OPERATOR),
            SWAP => Ok(&SWAP_OPERATOR),
            LASTOP => Ok(&LASTOP_OPERATOR),
            LASTARGS => Ok(&LASTARGS_OPERATOR),
            UNDO => Ok(&UNDO_OPERATOR),
            REDO => Ok(&REDO_OPERATOR),
            CLEAR => Ok(&CLEAR_OPERATOR),
            s => Err(PascalineError::OperatorSymbolError(s)),
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Operator::Add => 2,
            Operator::Sub => 2,
            Operator::Mul => 2,
            Operator::Div => 2,
            Operator::EucDiv => 2,
            Operator::Mod => 2,
            Operator::Pow => 2,
            Operator::Neg => 1,
            Operator::Sin => 1,
            Operator::Cos => 1,
            Operator::Tan => 1,
            Operator::ArcSin => 1,
            Operator::ArcCos => 1,
            Operator::ArcTan => 1,
            Operator::Sqrt => 1,
            Operator::Exp => 1,
            Operator::Ln => 1,
            // Operator::Num => 1,
            // Operator::Den => 1,
            // Operator::Complex => 2,
            // Operator::Re => 1,
            // Operator::Im => 1,
            // Operator::Arg => 1,
            // Operator::Norm => 1,
            Operator::Eq => 2,
            Operator::Neq => 2,
            Operator::Le => 2,
            Operator::Lt => 2,
            Operator::Ge => 2,
            Operator::Gt => 2,
            Operator::And => 2,
            Operator::Or => 2,
            Operator::Not => 1,
            Operator::Dup => 0,
            Operator::Drop => 0,
            Operator::Swap => 0,
            Operator::LastOp => 0,
            Operator::LastArgs => 0,
            Operator::Undo => 0,
            Operator::Redo => 0,
            Operator::Clear => 0
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            Operator::Add => ADD,
            Operator::Sub => SUB,
            Operator::Mul => MUL,
            Operator::Div => DIV,
            Operator::EucDiv => EUC_DIV,
            Operator::Mod => MOD,
            Operator::Pow => POW,
            Operator::Neg => NEG,
            Operator::Sin => SIN,
            Operator::Cos => COS,
            Operator::Tan => TAN,
            Operator::ArcSin => ARCSIN,
            Operator::ArcCos => ARCCOS,
            Operator::ArcTan => ARCTAN,
            Operator::Sqrt => SQRT,
            Operator::Exp => EXP,
            Operator::Ln => LN,
            // Operator::Num => NUM,
            // Operator::Den => DEN,
            // Operator::Complex => COMPLEX,
            // Operator::Re => RE,
            // Operator::Im => IM,
            // Operator::Arg => ARG,
            // Operator::Norm => NORM,
            Operator::Eq => EQ,
            Operator::Neq => NEQ,
            Operator::Le => LE,
            Operator::Lt => LT,
            Operator::Ge => GE,
            Operator::Gt => GT,
            Operator::And => AND,
            Operator::Or => OR,
            Operator::Not => NOT,
            Operator::Dup => DUP,
            Operator::Drop => DROP,
            Operator::Swap => SWAP,
            Operator::LastOp => LASTOP,
            Operator::LastArgs => LASTARGS,
            Operator::Undo => UNDO,
            Operator::Redo => REDO,
            Operator::Clear => CLEAR
        }
    }

    pub fn operate(&self, operands: &Vec<Token>) -> Result<Token, PascalineError> {
        let nb_operands = operands.len();
        let arity = self.arity();

        // First, check we can unpack enough numbers
        if nb_operands != arity {
            Err(PascalineError::ArityError {
                op: self.symbol(),
                expected: arity,
                found: nb_operands
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
                },
                Operator::EucDiv => Err(PascalineError::NotImplementedError),
                Operator::Mod => Err(PascalineError::NotImplementedError),
                Operator::Pow => Err(PascalineError::NotImplementedError),
                Operator::Neg => Err(PascalineError::NotImplementedError),
                Operator::Sin => Err(PascalineError::NotImplementedError),
                Operator::Cos => Err(PascalineError::NotImplementedError),
                Operator::Tan => Err(PascalineError::NotImplementedError),
                Operator::ArcSin => Err(PascalineError::NotImplementedError),
                Operator::ArcCos => Err(PascalineError::NotImplementedError),
                Operator::ArcTan => Err(PascalineError::NotImplementedError),
                Operator::Sqrt => Err(PascalineError::NotImplementedError),
                Operator::Exp => Err(PascalineError::NotImplementedError),
                Operator::Ln => Err(PascalineError::NotImplementedError),
                // Operator::Num => Err(PascalineError::NotImplementedError),
                // Operator::Den => Err(PascalineError::NotImplementedError),
                // Operator::Complex => Err(PascalineError::NotImplementedError),
                // Operator::Re => Err(PascalineError::NotImplementedError),
                // Operator::Im => Err(PascalineError::NotImplementedError),
                // Operator::Arg => Err(PascalineError::NotImplementedError),
                // Operator::Norm => Err(PascalineError::NotImplementedError),
                Operator::Eq => Err(PascalineError::NotImplementedError),
                Operator::Neq => Err(PascalineError::NotImplementedError),
                Operator::Le => Err(PascalineError::NotImplementedError),
                Operator::Lt => Err(PascalineError::NotImplementedError),
                Operator::Ge => Err(PascalineError::NotImplementedError),
                Operator::Gt => Err(PascalineError::NotImplementedError),
                Operator::And => Err(PascalineError::NotImplementedError),
                Operator::Or => Err(PascalineError::NotImplementedError),
                Operator::Not => Err(PascalineError::NotImplementedError),
                Operator::Dup => Err(PascalineError::NotImplementedError),
                Operator::Drop => Err(PascalineError::NotImplementedError),
                Operator::Swap => Err(PascalineError::NotImplementedError),
                Operator::LastOp => Err(PascalineError::NotImplementedError),
                Operator::LastArgs => Err(PascalineError::NotImplementedError),
                Operator::Undo => Err(PascalineError::NotImplementedError),
                Operator::Redo => Err(PascalineError::NotImplementedError),
                Operator::Clear => Err(PascalineError::NotImplementedError)
            }
        }
    }

    fn are_numbers(operands: &Vec<Token>) -> bool {
        operands.iter().all(|t| t.is_number())
    }

    fn unpack_one(operands: &Vec<Token>) -> Float {
        // Only used once the size has been checked so it shouldn't be an issue
        operands.first().and_then(|t| t.value()).unwrap()
    }

    fn unpack_two(operands: &Vec<Token>) -> (Float, Float) {
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
