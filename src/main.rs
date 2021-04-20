//Cryptarithmetic solver that finds numbers that satisfy an equation if substituted
//For example, ODD+ODD=EVEN or SEND+MORE=MONEY

use std::io;

use cryptarithmetic::solve_problem;

fn main() {
    let mut input = String::new();
    let threads = 6;

    println!("Enter Equation: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading info");

    //Function will print all answers in dict format
    solve_problem(input, threads);
}
