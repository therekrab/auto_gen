mod command;
mod parse;
mod tokens;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use command::GroupKind;
use parse::Parser;
use tokens::Tokenizer;

pub fn run(args: &mut impl Iterator<Item = String>) -> Result<(), String> {
    // First argument
    args.next();
    let Some(file_path) = args.next() else {
        println!("auto_gen version 1.0.0");
        return Err(String::from("Err: no file path supplied"));
    };
    let file = File::open(file_path).map_err(|err| err.to_string())?;
    let rdr = BufReader::new(file);
    let mut all_commands = Vec::new();
    for (i, line) in rdr.lines().enumerate() {
        let line = line.map_err(|err| err.to_string())?;
        let tokenizer = Tokenizer::new(&line, i + 1);
        let tokens = tokenizer.tokenize();
        if tokens.is_empty() {
            continue;
        }
        let mut parser = Parser::new(tokens);
        let expr = parser.expression();
        let cmd = expr.produce();
        all_commands.push(cmd);
    }
    let command = GroupKind::Sequential.group(&all_commands);
    println!("{}", command::finalize_json(&command.to_json()));
    Ok(())
}
