use super::token::Token;
use crate::errors::PascalineError;


#[derive(Debug)]
pub struct Stack<'a> {
    stack: Vec<Token<'a>>
}

const STACK_CAPACITY: usize = 4096;

// Computation stack, implementing a RPN logic
impl<'a> Stack<'a> {
    pub fn new() -> Stack<'a> {
        Stack {
            stack: Vec::with_capacity(STACK_CAPACITY)
        }
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn clear(&mut self) {
        self.stack.clear()
    }

    pub fn push(&mut self, token: &'a Token) -> Result<(), PascalineError> {
        let stack_size = self.stack.len();

        // If stack is full, throw error
        if stack_size >= STACK_CAPACITY {
            Err(PascalineError::FullStackError)
        } else {
            // If the token is an operator, then we need to pop some elements and run the operator
            // Results will be pushed to the stack
            match token {
                &Token::Operator(op) => {
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
                },
                &Token::Ignored => Err(PascalineError::TypeError),
                &t => Ok(self.stack.push(t))
            }
        }
    }
}
