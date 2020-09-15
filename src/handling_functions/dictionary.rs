use std::io::prelude::*;
use std::fs::OpenOptions;
use std::string::String;
use std::io;
use std::convert::From;

pub fn write_in_file ( mut t_word: String, nt_string: String ) -> io::Result<()> {
    let mut file_ptr = OpenOptions::new()
                .append(true)
                .read(true)
                .create(true)
                .open("../../dictionary.txt")?;
    t_word = prepares_dictionary_entry( nt_string, t_word );
    file_ptr.write_all(t_word.as_bytes())?; 
    Ok(())
}

fn prepares_dictionary_entry ( mut nt_string: String, t_word: String) -> String {
    nt_string = remove_line_skip_char(nt_string);
    nt_string += " = ";
    nt_string += &t_word;
    nt_string.push('\n');
    println!("{:?}", nt_string);
    nt_string
}

fn remove_line_skip_char ( word: String ) -> String{
    let unclean_word_as_vec = word.chars().collect::<Vec<char>>();
    let mut clean_word_as_vec = Vec::with_capacity( unclean_word_as_vec.len());
    for letter in unclean_word_as_vec {
        if letter.is_alphabetic() {
            clean_word_as_vec.push( letter );
        }
        else{
            continue;
        }
    }
    _parse_char_vec_to_string( clean_word_as_vec )
}

fn _parse_char_vec_to_string( word_as_vec: Vec<char> ) -> String{
    let mut boxed_string: String = String::from("");
    for letter in word_as_vec{
        boxed_string.push(letter);
    }
    boxed_string
}