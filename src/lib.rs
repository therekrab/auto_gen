mod command;
mod parse;
mod tokens;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use parse::Parser;
use tokens::Tokenizer;

pub fn run(args: &mut impl Iterator<Item = String>) -> Result<(), String> {
    // First argument
    args.next();
    let file_path = args.next().ok_or("File path not supplied.")?;
    let file = File::open(file_path).map_err(|err| err.to_string())?;
    let rdr = BufReader::new(file);
    let mut tokenizer;
    let mut parser;
    for line in rdr.lines() {
        let line = line.map_err(|err| err.to_string())?;
        tokenizer = Tokenizer::new(&line);
        let tokens = tokenizer.tokenize();
        println!("tokens: {tokens:?}");
        parser = Parser::new(tokens.clone());
        let expr = parser.expression();
        let cmd = expr.produce().pretty();

        println!("parse as: {cmd:?}");
    }
    Ok(())
}
