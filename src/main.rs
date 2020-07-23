use mathy::expr::prelude::*;
use mathy::calc::prelude::*;
use mathy::analysis::limit::*;

fn main() {
    let myexp = 1.0 / var('x'); 
    println!("f(x) = {}", myexp.to_string());

    let deriv = derive(myexp, 'x');
    let finalresult = simplify(deriv);
    println!("f'(x) = {}", finalresult.to_string());

    limit(finalresult.clone(), 'x', 0.0, 5);
}
