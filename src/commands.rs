/**
 * Core syntax model for the mini-Stata interpreter.
 *
 * This file defines the enums used across the project to represent parsed
 * Stata-like commands and comment markers. The parser produces these values,
 * and the evaluator consumes them to generate Python code.
 *
 * In practice, this is the shared command vocabulary for the whole compiler
 * pipeline.
 */
/**
 * The enum defines the supported commands to the 
 */
#[derive(Debug)]
pub enum StataCommand{
    Use(String),
    Summarize(Vec<String>),
    Describe,
    Generate{
        name:String,
        expr: String
    },
    Regresion{
        y:String,
        regressors: Vec<String>,
        options: Vec<String>

    },
    Browse(Vec<String>)
}

#[allow(dead_code)]
pub enum Comment {
    Star,
    DoubleSlash,
    SlashStar,
}
