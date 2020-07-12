use mathy::calc::expr::*;

fn main() {
    let myexp = add(var('x'), con(3.0));
    println!("Expression: {:?}", myexp);
    println!("x=7 -> {:?}", evaluate(myexp, 'x', 7.0));
}
