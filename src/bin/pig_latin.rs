use std::io;

fn main() {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    println!("Pig Latin Excercise");

    loop {
        println!("Please introduce a word");

        let mut word = String::new();

        io::stdin()
            .read_line(&mut word)
            .expect("Please introduce a word");

        let word = word.trim();

        if !word.chars().all(char::is_alphabetic) {
            println!("Type a word without number or spaces");
            continue;
        }

        let first_letter = word.chars().nth(0).unwrap();
        println!("First letter: {}", first_letter);
        if vowels.iter().any(|vowel| *vowel == first_letter) {
            println!("Pig latin word: {}hay", word);
        } else {
            let word = &word[1..word.len()];
            println!("Pig latin word: {}{}ay", word, first_letter);
        }
    }
}
