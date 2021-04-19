use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

//Takes input, generates permutations, filters obviously wrong ones, and then tests the rest of them
pub fn solve_problem(input: String) {
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


struct PermGenerator {
    pool: [u8; 10],
    indicies: [u8; 10],
    cycles: Vec<usize>,
    letter_count: usize,
}

impl PermGenerator {
    fn new(letter_count: usize) -> PermGenerator {
        let mut cycles: Vec<usize> = Vec::new();
        for i in (10 - letter_count + 1..11).rev() {
            cycles.push(i)
        }
        PermGenerator {
            pool: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            indicies: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            cycles: cycles,
            letter_count: letter_count,
        }
    }
}

impl Iterator for PermGenerator {
    type Item = Vec<u8>;

    //Permutation Cycle Algorithm inspired by Python itertools
    fn next(&mut self) -> Option<Self::Item> {
        for i in (0..self.letter_count).rev() {
            self.cycles[i] -= 1;
            if self.cycles[i] == 0 {
                let x = self.indicies;
                let mut j = i;
                for h in i + 1..10 {
                    self.indicies[j] = x[h];
                    j += 1;
                }
                for h in i..i + 1 {
                    self.indicies[j] = x[h];
                    j += 1;
                }

                self.cycles[i] = 10 - i;
            } else {
                let j = self.cycles[i];
                self.indicies.swap(i, 10 - j);
                let v: Vec<u8> = (0..self.letter_count)
                    .map(|i| self.pool[self.indicies[i] as usize])
                    .collect();
                return Some(v);
            }
        }
        None
    }
}

struct Equation {
    //infix: String,
    postfix: VecDeque<String>,
    vars: Vec<char>,
    first_vars_index: Vec<usize>,
}

impl Equation {
    //Uses shunting yard algorithm to convert infix (normal) equation into postfix (computer readable)
    //Note: algo mainly written for learning purposes, should probably migrate to an existing crate for this next time
    //Ignores any symbol that isn't a letter (e.g. 'A','B'etc.) or an operator('+', '*', etc.)
    //
    //Also outputs variables used: vars and first_vars_index. First_vars_index gives indicies for variables in vars that can not be 0 because
    //they are at the front of a number
    fn new(infix: String) -> Equation {
        let mut operator_precedence = HashMap::new();
        operator_precedence.insert('+', 2);
        operator_precedence.insert('-', 2);
        operator_precedence.insert('/', 3);
        operator_precedence.insert('*', 3);
        operator_precedence.insert('=', 1);

        let letters = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let mut variables = HashSet::new();
        let mut first_vars = HashSet::new();
        let mut output_queue = VecDeque::new();

        let mut operator_stack = Vec::new();
        let mut current_word = String::new();   //Placeholder var to combine chars that form a single "number"

        for token in infix.chars() {
            //Extra Logic that groups contiguous chars into a single "number"
            if letters.contains(&token) {
                variables.insert(token);
                current_word.push(token);
                continue;
            } else {
                if current_word.len() != 0 {
                    first_vars.insert(current_word.chars().nth(0).unwrap());
                    output_queue.push_back(current_word);
                    current_word = String::new();
                }
            }

            if operator_precedence.contains_key(&token) {
                let precedence = operator_precedence[&token];
                //Note: How can I make this combined while statement more readable?
                while operator_stack.len() != 0
                    && operator_precedence.contains_key(operator_stack.last().unwrap())
                    && operator_precedence[operator_stack.last().unwrap()] > precedence
                {
                    output_queue.push_back(operator_stack.pop().unwrap().to_string());
                }
                operator_stack.push(token);
            } else if token == '(' {
                operator_stack.push(token);
            } else if token == ')' {
                while *operator_stack.last().unwrap() != '(' {
                    output_queue.push_back(operator_stack.pop().unwrap().to_string());
                }
                operator_stack.pop();
            }
        }

        if current_word.len() != 0 {
            output_queue.push_back(current_word);
        }

        for _i in 0..operator_stack.len() {
            output_queue.push_back(operator_stack.pop().unwrap().to_string());
        }

        let vars: Vec<char> = variables.into_iter().collect();
        let mut first_vars_index: Vec<usize> = vec![];
        for (index, var) in vars.iter().enumerate() {
            if first_vars.contains(var) {
                first_vars_index.push(index);
            }
        }

        Equation {
            //infix: infix,
            postfix: output_queue,
            vars: vars,
            first_vars_index: first_vars_index,
        }
    }

    //Given a hashmap of mapping the variable names to numbers, the function checks if the equation is true or false
    pub fn eval(&self, var_value: &HashMap<char, u8>) -> u64 {
        let mut postfix_copy = self.postfix.clone();

        let mut calc_stack: Vec<u64> = vec![];
        loop {
            let token = match postfix_copy.pop_front() {
                Some(n) => n,
                None => break,
            };
            let new_stack_value = match token.as_str() {
                "+" => {
                    let b = calc_stack.pop().unwrap();
                    let a = calc_stack.pop().unwrap();
                    a + b
                }
                "-" => {
                    let b = calc_stack.pop().unwrap();
                    let a = calc_stack.pop().unwrap();
                    a - b
                }
                "*" => {
                    let b = calc_stack.pop().unwrap();
                    let a = calc_stack.pop().unwrap();
                    a * b
                }
                "/" => {
                    let b = calc_stack.pop().unwrap();
                    let a = calc_stack.pop().unwrap();
                    a / b
                }
                //Returns 1 and 0 instead of true/false because type must be u64
                "=" => {
                    let b = calc_stack.pop().unwrap();
                    let a = calc_stack.pop().unwrap();
                    if a == b {1} else {0}
                }
                //If not an operator, convert to number and push to stack
                _ => {
                    let mut value = 0;
                    for (idx, var) in token.chars().rev().enumerate() {
                        value += 10_u64.pow(idx as u32) * var_value[&var] as u64;
                    }
                    value
                }
            };
            calc_stack.push(new_stack_value);
        }
        calc_stack[0]
    }
}
