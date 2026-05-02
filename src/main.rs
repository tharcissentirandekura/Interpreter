/**
 * Entry point for the mini-Stata prototype.
 *
 * This file wires the full pipeline together:
 * 1. defines a small sample of Stata-like commands,
 * 2. parses each line into `StataCommand` values,
 * 3. translates the parsed commands into Python,
 * 4. sends the generated script to Python over stdin,
 * 5. executes it with the local virtualenv Python runtime.
 *
 * In short, `main.rs` is the orchestration layer for the parser and evaluator.
 */
use std::{
    io::Write,
    process::{Command, Stdio},
};

mod commands;
mod evaluator;
mod parser;
use crate::commands::StataCommand;
use evaluator::convert_to_target;
use parser::parse_line;

fn main() {
    /*
     * To test our current mini Stata interpreter, we can add some code
     * Then,parse it and evaluate it
     */
    let stata_code = r#"
        use econmath.dta
        // summarize
        * ignore this line
        browse age work
        describe 
        gen lscore = log(score)
        gen expcore = exp(score)
        gen sqtcore = sqrt(score)
        describe
        reg lscore age work study,robust
        
    "#;
    //Parse each line by filter and mapping each line to parse_line(parser)
    // SO, send each line into a parser
    let commands:Vec<StataCommand> = stata_code
        .lines()
        .filter_map(parse_line)
        .collect();
    println!("Commands: {:?}",commands);
    // Interpet the code to our target language
    let target_code = convert_to_target(commands);
    // Execute the generated Python without leaving a persistent script on disk.
    let mut child = Command::new("./venv/bin/python")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start the Python interpreter");

    child
        .stdin
        .as_mut()
        .expect("Failed to open stdin for the Python interpreter")
        .write_all(target_code.as_bytes())
        .expect("Failed to send generated code to Python");

    let status = child
        .wait()
        .expect("Failed to run the generated Python code");
    println!("The code exited with: {}",status);

}
