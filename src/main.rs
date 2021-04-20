//Cryptarithmetic solver that finds numbers that satisfy an equation if substituted
//For example, ODD+ODD=EVEN or SEND+MORE=MONEY

use std::io;

use cryptarithmetic::solve_problem;

fn main() {
    let mut input = String::new();
    let thread_count = 6;

    println!("Enter Equation: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading info");

    //Function will return a vector of all answers in a dict format
    let answers = solve_problem(input, thread_count);
    for ans in answers {
        println!("{:?}", ans);
    }
}
