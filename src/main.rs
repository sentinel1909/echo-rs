// src/main.rs

/*
a very primitive version of the utility "echo", written in Rust
takes a user supplied argument and echos it back to the command line
if the user supplies an empty argument, the a newline is output to the command line
*/

// dependencies
use std::env;
use std::io::{self, Write};

// function to parse the input
fn parse_input(value: Vec<String>) -> String {
    
    // check if there is a user supplied value, if not, return a string containing the a newline, otherwise, return the entered value as a string
    if value.len() == 1 {
        let echoed_value = "\n";
        echoed_value.to_string()
    } else {
        let echoed_value = &value[1];
        echoed_value.to_string()
    }
}

// function to write the output
fn echo_output(value: String) {
    // get a stdout entity and lock it for output
    let mut stdout = io::stdout();

    // write the output
    write!(stdout, "{} \n", value).expect("Could not write to stdout...");
}

// main function
fn main() {
    // get the value supplied by the user
    let input: Vec<String> = env::args().collect();

    // call the echo function which checks the users input and writes either a newline (empty input) or write the value provided
    echo_output(parse_input(input));
}
