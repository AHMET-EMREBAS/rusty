use std::fs::File;

fn main() {
    // RUST_BACKTRACE=1 ./error_handling.exe
    // RUST_BACKTRACE=full ./error_handling.ex
    // panic!("Crash and burn");

    let greeting_file_result = File::open("hello.txt");

    let result = match greeting_file_result {
        Ok(_) => println!("File exist"),
        Err(_) => println!("File Not exist"),
    };

    println!("{:?}", greeting_file_result)
}
