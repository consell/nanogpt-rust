use std::fs::File;
use std::io;
use std::io::Read;

use crate::bigram::run;
use crate::tokenizer::Tokenizer;
// use std::prelude::*;

mod bigram;
mod tokenizer;

fn load_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() -> Result<(), std::io::Error> {
    let contents = load_file("./data/input.txt")?;
    // let tokenizer = Tokenizer::new(&contents);
    run(&contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer() {
        let contents = load_file("./data/input.txt").unwrap();
        let tokenizer = Tokenizer::new(&contents);
        
        assert_eq!(tokenizer.decode(&tokenizer.encode(&contents)), contents);
    }
}