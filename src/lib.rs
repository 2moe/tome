//! Convert strings to Pig Latin

/// <https://rustwiki.org/en/book/ch08-03-hash-maps.html>  
/// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!  

/// 将字符串转换为 Pig Latin。  
/// 每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，  
/// 元音字母开头的单词则在结尾增加 “hay”。  

// ^(a|e|i|o|u)(.*) -> \1\2-hay,
/// annie -> annie-hay,  
/// last -> ast-lay,  
/// fire -> ire-fay  

pub fn print_pig_latin(sentence_raw: &str) {
    sentence_raw.split_whitespace().for_each(|word| {
        let word_prefix: &str = &word[..1].to_lowercase();

        match word_prefix {
            "a" | "e" | "i" | "o" | "u" => {
                print!("{}-hay ", word);
            }
            "," | "." | "!" | "?" | "/" => print!("{} ", word),
            _ => {
                let new_word_1: &str = &word[1..];
                let new_word_2: String = format!("-{}ay", word_prefix);
                let new_word_full: String = format!("{}{}", new_word_1, new_word_2);

                print!("{} ", new_word_full);
            }
        }
    });
    println!("");
}
