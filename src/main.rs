use mathy::calc::prelude::*;

fn main() {
    let myexp = tan(var('x')); 
    println!("f(x) = {}", myexp.to_string());

    let deriv = derive(myexp, 'x');
    println!("f'(x) = {}", deriv.to_string());
}
