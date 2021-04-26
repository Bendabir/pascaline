mod core;
mod calculator;
mod errors;

use crate::core::Parser;

fn main(){
    let text = "1 + 2 - 5.5 * 1235";
    let parser = Parser;
    let tokens = parser.parse(text);

    println!("{:?}", tokens);
}
