use std::collections::HashSet;

pub fn number_of_special_chars(word: String) -> i32 {
    let mut special_set: HashSet<char> = HashSet::new();
    let mut size: i32 = 0;
    let _: Vec<_> = word
        .chars()
        .map(|s| {
            if s.is_uppercase() {
                if special_set.contains(&s.to_ascii_lowercase()) && !special_set.contains(&s) {
                    size += 1;
                }
            } else {
                if special_set.contains(&s.to_ascii_uppercase()) && !special_set.contains(&s) {
                    size += 1
                }
            }
            special_set.insert(s);
        })
        .collect();
    //let size: i32 = setmap.len() as i32;
    size
}

fn main() {
    let word: String = "aaAbcBC".to_owned();

    println!("{}", number_of_special_chars(word));
}
