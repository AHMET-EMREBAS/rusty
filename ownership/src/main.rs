fn main() {
    let mut s1 = String::from("S1 text value  >> ");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    println!("The value of the s1 is {s1}");
    change(&mut s1);

    println!("The first word of the s1 is {s1}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_str: &mut String) {
    some_str.push_str(", Sub string");
}
