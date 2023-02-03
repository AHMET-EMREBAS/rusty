fn main() {
    let text = String::from("some text here");

    get_size(&text);
    get_size(&text);
    let updated_text = update_text(text);
    finalize_text(updated_text);

    println!("-------------------------------");

    let some_text = String::from("012345 78911");
    let first_word = &some_text[0..6];
    let second_word = &some_text[7..12];
    println!("First word is {first_word}");
    println!("Second word is {second_word}");
    println!("Whole text is {some_text}");
    let first_word_value = get_first_word(&some_text);

    println!("First word of {some_text} is {}", first_word_value)
}

/*
Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
*/

/*
    - No mutation
*/
fn get_size(value: &String) -> usize {
    value.len()
}

/*
    Last owner of the variable
    Final statement
    We cannot access the value after giving the ownership to this fn.
*/
fn finalize_text(value: String) {
    let mut result = String::from("");

    let prefix = ":".repeat(5);
    let suffix = ":".repeat(5);
    let tsize = get_size(&value);

    // The &value syntax lets us create a reference that refers to the value of value but does not own it.
    result.push_str(&prefix);
    result.push_str(&value);
    result.push_str(&suffix);
    result.push_str(&tsize.to_string());

    let (text_size, mut text_value) = value_size(value);

    text_value.push_str("Chaning dangeraous");

    println!("Size of {} is  {}", text_value, text_size);

    println!("Final Text");
    println!("{result}");
    println!("You cannot access the value anymore.");
}

fn value_size(value: String) -> (usize, String) {
    return (value.len(), value);
}

// Getting ownership and mutable access
fn update_text(mut value: String) -> String {
    value.push_str("--- Updated");
    value
}

fn get_first_word(text: &String) -> &str {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[0..i];
        }
    }

    &text[..]
}
