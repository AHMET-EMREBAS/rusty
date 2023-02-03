use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    // RUST_BACKTRACE=1 ./error_handling.exe
    // RUST_BACKTRACE=full ./error_handling.ex
    // panic!("Crash and burn");

    let file_path = "files/hello.txt";

    let greeting_file_result = File::open(&file_path);

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_path) {
                Ok(fc) => fc,
                Err(e) => panic!("Could not create the file!"),
            },
            others => panic!("Could not open the file!, {:?}", others),
        },
    };

    let mut file_content = String::from("");
    greeting_file
        .read_to_string(&mut file_content)
        .expect("Could not read the file");

    println!("File Content is {}", file_content);

    let file_content = read_file_content(file_path).expect("Not read!");

    println!("File content : {:?}", file_content);
}

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let mut file_content = String::new();

    File::open(file_path)?.read_to_string(&mut file_content)?;
    Ok(file_content)
}
