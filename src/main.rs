mod core;
mod calculator;
mod errors;

use crate::core::Parser;
use crate::core::Stack;
use crate::core::Token;

fn main(){
    let mut stack = Stack::new();

    let t1 = Token::new_integer(2);
    let t2 = Token::new_integer(3);
    let t3 = Token::new_operator("+").unwrap();
    let t4 = Token::new_float(2.5);
    let t5 = Token::new_operator("-").unwrap();
    let t6 = Token::new_operator("*").unwrap();

    stack.push(&t1).unwrap();
    stack.push(&t2).unwrap();

    println!("{:?}", stack);
    println!("\t{} elements", stack.size());

    stack.push(&t3).unwrap();

    println!("{:?}", stack);
    println!("\t{} elements", stack.size());

    stack.push(&t4).unwrap();

    println!("{:?}", stack);
    println!("\t{} elements", stack.size());

    stack.push(&t5).unwrap();

    println!("{:?}", stack);
    println!("\t{} elements", stack.size());

    stack.push(&t1).unwrap();

    println!("{:?}", stack);
    println!("\t{} elements", stack.size());

    stack.push(&t6).unwrap();

    println!("{:?}", stack);
    println!("\t{} elements", stack.size());
}
