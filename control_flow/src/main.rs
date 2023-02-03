fn main() {
    let is_true = true;

    if is_true {
        println!("IS true");
    } else if is_true != true {
        println!("IS true");
    }

    println!("IS true");

    let mut counter = 0;
    let result = 'countup: loop {
        counter += 1;
        if counter == 10 {
            break 'countup counter * 2;
        }
    };

    println!("Result: {result}");

    for i in [1, 2, 3, 4, 5] {
        println!("Hello there {i}");
    }

    for b in (1..4).rev() {
        println!("{b}");
    }
}
