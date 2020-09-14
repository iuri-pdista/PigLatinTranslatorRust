use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::string::String;
use std::io;
use std::convert::From;

fn main() {
    let mut nt_word = String::new();
    io::stdin()
        .read_line(&mut nt_word)
        .expect("Failed to read line");
    println!("{:?}", nt_word);
    {
        let nt_word = nt_word;
        let mut nt_word_in_char: Box<Vec<char>> = Box::new(nt_word.chars().collect::<Vec<char>>());
        let t_word_in_char: Box<Vec<char>> = translate(nt_word_in_char);
        let _result: io::Result<()> = write_in_file(t_word_in_char);
    }
}