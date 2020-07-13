use mathy::calc::prelude::*;

fn main() {
    let myexp = (var('x') * con(3.0)) / con(2.0); 

    println!("Expression: {:?}", myexp);
    println!("f(x) = {:?}", evaluate(myexp, 'x', 7.0));
}
