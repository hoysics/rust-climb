fn main() {
    println!("Hello, world!");
    let mut s = String::from("Hello, world!");
    let word = fisrt_word(&s);
    s.clear();
    println!("first word: {}", word);
}

fn fisrt_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
