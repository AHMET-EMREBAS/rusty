fn main() {
    let mut s = String::from("Hello World");

    let len = s.len();
    let slice = &s[0..5];
    println!("{slice}");
    let slice = &s[..];
    println!("{slice}");

    let n_string = first_word(&s);
    println!("{n_string}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
