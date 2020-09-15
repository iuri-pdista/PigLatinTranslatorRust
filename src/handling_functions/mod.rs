pub mod translation;
pub mod dictionary;

use std::string::String;
use std::io;

pub fn translate ( nt_word: String ){
    let nt_word_in_char: Box<Vec<char>> = Box::new(nt_word.chars().collect::<Vec<char>>());
    let t_word: String = translation::transform_word(nt_word_in_char);
    let _result: io::Result<()> = dictionary::write_in_file(t_word, nt_word);
}