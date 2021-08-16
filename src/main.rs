use std::io;
use tome::print_pig_latin;

fn main() {
    println!("Please input a sentence:");
    let mut input_sentence: String = String::new();
    io::stdin()
        .read_line(&mut input_sentence)
        .expect("Fail to read input line");
    let sentence_raw: &str = input_sentence.trim();
    print_pig_latin(sentence_raw);
}
