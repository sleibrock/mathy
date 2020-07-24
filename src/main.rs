use mathy::expr::prelude::*;
use mathy::analysis::newton::*;

fn main() {
    let sqrt = powf(var('x'), 2.0) - con(612.0);

    println!("{:?}", sqrt);

    newton(sqrt, 'x', 10.0, 10);
}
