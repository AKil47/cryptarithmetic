//Cryptarithmetic solver that finds numbers that satisfy an equation if substituted
//For example, ODD+ODD=EVEN or SEND+MORE=MONEY

use std::io;

use cryptarithmetic::solve_problem;

fn main() {
    let mut input = String::new();
    let mut thread_count = String::new();

    println!("Enter Equation: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading info");

    println!("Enter Number of Threads: ");
    io::stdin()
        .read_line(&mut thread_count)
        .expect("Error reading thread info");

    let thread_count: usize = match thread_count.parse() {
        Ok(n) => n,
        Err(_) => 6 //Default thread count is 6
    };

    //Function will return an Option which will have vector of all answers in a dict format if there are any
    match solve_problem(input, thread_count) {
        None => println!("No solution found"),
        Some(answers) => {
            for answer in answers {
                println!("{:?}", answer);
            }
        }
    }
    
}
