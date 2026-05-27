use collection::HashSet;

pub fn number_of_special_chars(word: String) -> i32 {
    let mut setmap: HashSet<&str> = HashSet::new();
    let size: i32 = 0;
    word.map(|s| {
        setmap.push(s);
    })
    size += setmap.len;
    size
}

fn main() {
    let word: String = "aaAbcBC".to_owned();
    
    println!("{}", number_of_special_chars(word));
}

