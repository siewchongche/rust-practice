// Convert strings to pig latin. The first consonant of each word is 
// moved to the end of the word and “ay” is added, so “first” becomes 
// “irst-fay.” Words that start with a vowel have “hay” added to the end 
// instead (“apple” becomes “apple-hay”). Keep in mind the details about 
// UTF-8 encoding!

use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    println!("{}", "Input your text");

    let mut strings = String::new();
    io::stdin()
        .read_line(&mut strings)?;
        // .expect("failed to read line");

    let vowel = "aeiou";

    let mut pig_latin = String::new();
    for element in strings.split(' ') {
        let first_letter = &element[..1];
        if vowel.contains(first_letter) {
            pig_latin.push_str(element);
            pig_latin.push_str("-hay");
        } else {
            let word_without_first_letter = &element[1..];
            pig_latin.push_str(word_without_first_letter);
            pig_latin.push_str("-");
            pig_latin.push_str(first_letter);
            pig_latin.push_str("ay");
        }
        pig_latin.push_str(" ");
    }

    println!("{}", pig_latin);

    Ok(())
}