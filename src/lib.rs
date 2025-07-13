mod tokens;

use std::{fs::File, io::{BufRead, BufReader}};

use tokens::Tokenizer;

pub fn run(args: &mut impl Iterator<Item = String>) -> Result<(), String> {
    // First one
    args.next();
    let file_path = args.next().ok_or("File path not supplied.")?;
    let file = File::open(file_path).map_err(|err| err.to_string())?;
    let rdr = BufReader::new(file);
    let mut tokenizer;
    for line in rdr.lines() {
        let line = line.map_err(|err| err.to_string())?;
        tokenizer = Tokenizer::new(&line);
        println!("{:?}", tokenizer.tokenize());
    }
    Ok(())
}
