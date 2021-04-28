use super::token::Token;
use super::operator::Operator;
use crate::errors::PascalineError;
use std::fmt;


#[derive(Debug)]
pub struct Stack<'a> {
    stack: Vec<Token<'a>>,
    last_op: Option<&'a Operator>
}

const STACK_CAPACITY: usize = 4096;

// Computation stack, implementing a RPN logic
impl<'a> Stack<'a> {
    pub fn new() -> Stack<'a> {
        Stack {
            stack: Vec::with_capacity(STACK_CAPACITY),
            last_op: None
        }
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn clear(&mut self) {
        self.stack.clear()
    }

    pub fn push(&mut self, token: Token<'a>) -> Result<(), PascalineError> {
        let stack_size = self.stack.len();

        // If stack is full, throw error
        if stack_size >= STACK_CAPACITY {
            Err(PascalineError::FullStackError)
        } else {
            // If the token is an operator, then we need to pop some elements and run the operator
            // Results will be pushed to the stack
            match token {
                Token::Operator(op) => {
                    // First, check for stack operators
                    let result = match op {
                        Operator::Dup => {
                            match self.stack.first() {
                                Some(&t) => Ok(self.stack.push(t)),
                                None => Err(PascalineError::EmptyStackError)
                            }
                        },
                        Operator::Drop => {
                            match self.stack.pop() {
                                Some(_) => Ok(()),
                                None => Err(PascalineError::EmptyStackError)
                            }
                        },
                        Operator::Swap => {
                            if stack_size < 2 {
                                Err(PascalineError::ArityError {
                                    op: op.symbol(),
                                    expected: 2,
                                    found: stack_size
                                })
                            } else {
                                Ok(self.stack.swap(stack_size - 1, stack_size - 2))
                            }
                        },
                        Operator::Clear => Ok(self.clear()),
                        Operator::LastOp => {
                            // Just push the last op to the stack to handle all the logic
                            match self.last_op {
                                None => Err(PascalineError::NoLastOperatorError),
                                Some(_) => Err(PascalineError::LastOperatorFlag)
                                // Some(o) => self.push(Token::new_operator(o.symbol()).unwrap())
                            }
                        },
                        // Otherwise, apply the operator's logic
                        _ => {

                            let arity = op.arity();

                            if stack_size < arity {
                                Err(PascalineError::ArityError {
                                    op: op.symbol(),
                                    expected: arity,
                                    found: stack_size
                                })
                            } else {
                                // Get the operands
                                let operands = self.stack.split_off(stack_size - arity);

                                // Try to run the operator
                                match op.operate(&operands) {
                                    // If it fails, recover the tokens in the stack
                                    Err(e) => {
                                        self.stack.extend(operands);
                                        Err(e)
                                    },
                                    // Otherwise, push the result
                                    Ok(t) => Ok(self.stack.push(t))
                                }
                            }
                        }
                    };

                    // Check the result before storing the last operator
                    // If something went bad, just return right away
                    // Otherwise, store the last op (if not the LASTOP op)
                    // This a bit of a dirty workaround because I couldn't find any elegant
                    // solution to avoid borrowing issues when running the recursion earlier
                    match result {
                        Err(PascalineError::LastOperatorFlag) => {
                            // Safe to unwrap because the flag tells us we can apply this operation
                            // Same for the created token, the operator is sure to be valid
                            let last_op = self.last_op.unwrap();
                            self.push(Token::new_operator(last_op.symbol()).unwrap())
                        },
                        Err(e) => Err(e),
                        Ok(v) => {
                            self.last_op = Some(op);
                            Ok(v)
                        }
                    }
                },
                Token::Ignored => Err(PascalineError::TypeError),
                t => Ok(self.stack.push(t))
            }
        }
    }

    pub fn result(&self) -> Option<&Token> {
        self.stack.first()
    }
}

impl<'a> fmt::Display for Stack<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "[{}]",
            self.stack.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(", ")
        )
    }
}
