mod handling_functions;

use std::string::String;
use std::io;

fn main() {
    let mut nt_word = String::new();
    io::stdin()
        .read_line(&mut nt_word)
        .expect("Failed to read line");
    handling_functions::translate(nt_word);
}