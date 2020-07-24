mathy.rs
========

A simple library for various math purposes. Currently a (huge) work in progress.

## Goals

The goal is to accomplish to a similar effect what I often write in functional programming languages as a thought experiment: differential calculus programs. It starts with creating a sum-type with recursive elements, where nodes are arithmetic operations and children can either be further arithmetic operators, or just raw values. It's a fun experiment to write a differentiatior to produce derivatives from simple equations.

The end-goal is to have something usable to make solving basic math and physics easier to solve with Rust. I would like to write library-ready code to find derivatives as well as anti-derivatives, and take advantage of Rust's `std::ops` traits to make it as easy to write equations as if you were writing Python using SymPy. Then extend the code to work in different fields of math or physics, like adding matrices, vectors/vector fields, complex function analysis and maybe weirder things like quaternions.

### Current Tasks

* Inverse/hypberbolic trigonometry functions
* Finish off analytical tools like Newton's Method, Limits, Sums, Products and Series
* Mapping of real-valued functions to work on complex numbers
* Differentiation of Complex-valued (basic) functions
* Partial derivatives
* Basic Integration rules


## Contributing

Please open an issue if there is anything you would like to see added or want to discuss helping.
