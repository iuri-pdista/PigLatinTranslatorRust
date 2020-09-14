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
    {
        let nt_word = nt_word;
        let mut _nt_word_in_char: Box<Vec<char>> = Box::new(nt_word.chars().collect::<Vec<char>>());
        let _t_word_in_char: String = translate(_nt_word_in_char);
        //let _result: io::Result<()> = write_in_file(t_word_in_char);
    }
}

/*
fn write_in_file ( t_word: Box<Vec<char>> ) -> io::Result<()> {
    let mut file_ptr = OpenOptions::new()
                .append(true)
                .read(true)
                .create(true)
                .open("../../dictionary.txt")?;
    let mut byte_string: String = String::from("");
    for letter in *t_word{
        byte_string.push(letter);
    }
    file_ptr.write_all(byte_string.as_bytes())?;
    Ok(())
}*/

fn translate ( nt_word: Box<Vec<char>> ) -> String{
    let unboxed_word: Vec<char> = *nt_word;
    let mut t_word: String = String::from("");
    {
        let first_letter = unboxed_word[0];
        t_word = push_suffix(t_word, first_letter);
        t_word = insert_word(unboxed_word, t_word)
    }
    t_word
}

fn insert_word( char_vec: Vec<char>, mut word: String ) -> String{
    let first_letter = char_vec[0];
    let mut count = 0;
    for letter in char_vec.iter(){
        if *letter == first_letter{
            continue;
        }            
        else{
            if (*letter).is_alphabetic() {
                word.insert( count, *letter);
                count += 1;
            }
            else{
                count += 1;
            }
        }
    }
    word
}

fn push_suffix ( mut word: String, first_letter: char ) -> String{
    word.push( first_letter );
    word.push('a');
    word.push('y');
    word
}