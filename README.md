# cryptarithmetic

A multithreaded tool written in Rust to solve Cryptarithmetic Problems. 

 Also known as Verbal Arithmetic or Cryptarithms, these are puzzles where mathematical equations have had their digits replaced with numbers. For example the equation `SEND+MORE=MONEY` can be rewritten as `9567 + 1085 = 10652`. Notice how each letter represented a digit and that the same digit appears in every place where its corresponding letter was. 
 Wikipedia Article: [Verbal Arithmetic](https://en.wikipedia.org/wiki/Verbal_arithmetic)

The program takes the equation as a string input and generates every possible solution to the problem based on the variables given. It then allocates these possible solutions to a worker thread(s) that check if these solutions are in fact valid. This project was created to help me learn about Rust and its features.


## Installation

The easiest way to use the program is to just clone the git repo. Afterwards, you can use cargo to build an executable.

```bash
git clone [Github URL]
cd cryptarithmetic
cargo build --release
cargo run --release
```

## Library Usage
The `main.rs` file is a simple front end that makes use of the logic. At its core, the solver can be invoked by calling `cryptarithmetic::solve_problems(input, thread_count)` where `input` is a `String` and `thread_count` is a `usize`.

```
let input = String::from("ODD+ODD=EVEN");
let thread_count: usize = 5;

//Call the solver
use cryptarithmetic::solve_problems;
let answers = solve_problems(input, thread_count);

//Go through each answer and print it
for answer in answers {
	println!("{:?}", answer);
}
```


## Contributing
Pull requests are always welcome! There is a brief todo list of things that I would like to implement. 

This project was mainly created to help me understand all of Rust's features. Please do not add any new crates that do not come with the standard installation of Rust. For right now, I would only like to use what the standard library provides so that I can get a better grasp of the language.

## To Dos:
1. Add comments that integrate with rustdoc R
2. Allow user to specify if they want output as dictionary (O: 8, D: 5, ...) or as filled equation (855+855) with flags or env vars
3. Refactor code so that each function only achieves a single purpose. (This could mean creating other files)
4. Add CLI input for number of threads
5. Add unit tests
6. Add support for other operations (^, !, %, etc.)