//Cryptarithmetic solver that finds numbers that satisfy an equation if substituted
//For example, ODD+ODD=EVEN or SEND+MORE=MONEY

use std::collections::HashMap;
use std::io;

use cryptarithmetic::Equation;
use cryptarithmetic::PermGenerator;

fn main() {
    let mut input = String::new();

    println!("Enter Equation: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading info");

    let equation = Equation::new(input);

    let perm_gen = PermGenerator::new(equation.vars.len())
        .filter(|perm| valid_perm(perm, &equation.first_vars_index));

    let mut var_value: HashMap<char, u8> = HashMap::new();
    for perm in perm_gen {
        for (var, value) in equation.vars.iter().zip(perm.iter()) {
            var_value.insert(*var, *value);
        }

        if let 1 = equation.eval(&var_value) {
            println!("{:?}", var_value);
        }
    }
}

//Eliminiate any perms which assign a 0 to a variable that starts a number
fn valid_perm(perm: &Vec<u8>, first_vars: &Vec<usize>) -> bool {
    for var in first_vars {
        if perm[*var] == 0 {
            return false;
        }
    }
    true
}
