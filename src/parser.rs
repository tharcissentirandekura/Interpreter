/**
 * Parser for the mini-Stata language.
 *
 * This file is responsible for turning one raw source line at a time into a
 * structured `StataCommand`. It also recognizes comment syntaxes and ignores
 * blank or commented lines before command parsing happens.
 *
 * The parser is intentionally small and command-specific: each supported
 * keyword such as `use`, `browse`, `gen`, or `reg` is matched directly and
 * converted into the enum defined in `commands.rs`.
 */
use crate::commands::{
    Comment,
    StataCommand
};

/**
 * TODO: add multople line parser
 * If the comment starts with /* */, we should parse until we see  a clsing which is
 */
pub fn detect_comment (line:&str) -> Option<Comment>{
    if line.starts_with("/*"){
        Some(Comment::SlashStar)
    }else if line.starts_with("//"){
        Some(Comment::DoubleSlash)
    }else if line.starts_with("*"){
        Some(Comment::Star)
    }else{
        None
    }
}
pub fn parse_line(line:&str) ->Option<StataCommand>{
    let line = line.trim();
    // empty line of comments
    if line.is_empty(){
        return None;
    }

    if line == "browse"{
        return Some(
            StataCommand::Browse(vec![])
        );
    }
    if line == "describe" {
        return Some(
            StataCommand::Describe
        )
    }

    // comment line: ignore
    // shold
    if detect_comment(line).is_some(){
        return None;
    }
    /*
     * parsing the browse command
     * Rest: rest is the remaining code afte the command
     */
    if let Some(rest) = line.strip_prefix("browse "){
        let vars = rest
            .split_whitespace()
            .map(|s|s.to_string())
            .collect();
        return Some(StataCommand::Browse(vars))
    

    }

    // parsing the file loader: use
    if let Some(rest) = line.strip_prefix("use "){
        return Some(
            StataCommand::Use(
                rest.trim().to_string()
            )
        );
    }
    /*
     * Parse the summarize command
     */
    if let Some(rest) = line.strip_prefix("summarize"){
        let variables = rest
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        return Some(
            StataCommand::Summarize(variables)
        )
    }

    /*
     * Parse the gen command
     * This comand creates a new variable and add it to our data set
     * Luckily, pandas does this very easily
     */

    if let Some(rest) = line.strip_prefix("gen ") {
        let parts:Vec<&str> = rest.split("=").collect();
        if parts.len() != 2{
            panic!("Invalid gen syntax:{}",line);
        }
        return Some(StataCommand::Generate {
             name: parts[0].trim().to_string(),
              expr: parts[1].trim().to_string(), 
        });
    }

    /*
     * Regression mode;
     */
    if let Some(rest) = line.strip_prefix("reg "){
        let pieces:Vec<&str> = rest.split(",").collect();
        let main_parts = pieces[0].trim();
        let options = if pieces.len() > 1 {
            pieces[1]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        }else{
            vec![]
        };

        let regressors:Vec<String> = main_parts
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        if regressors.len() < 2{
            panic!("reg needs one y variable and at least one x variable: {}", line);
        }
        return Some(
            StataCommand::Regresion { 
                y: regressors[0].clone(),
                regressors: regressors[1..].to_vec(), 
                options 
            }
        )
    }


    /*
     * Fpr teh unknow command, I am making it panic but this can be explored later
     */
    panic!("Unknown command: {}", line);
    // None
}
