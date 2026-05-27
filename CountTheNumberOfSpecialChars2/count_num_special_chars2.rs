pub fn number_of_special_chars(word: String) -> i32 {
    println!(
        "{}",
        word.find(b'a' as char).take() < word.find((b'A' - 32) as char).take()
    );
    (b'a'..=b'z')
        .filter(|&letter| {
            word.contains(letter as char)
                && word.contains((letter - 32) as char)
                && (word.rfind(letter as char).take() < word.find((letter - 32) as char).take())
        })
        .count() as i32
}

fn main() {
    let word: String = "aaAbcBC".to_owned();
    let word2: String = "AbBCab".to_owned();

    println!("{}", number_of_special_chars(word));
    println!("{}", number_of_special_chars(word2));
}
