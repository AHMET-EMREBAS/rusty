enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, world!");

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddrKind::V6(String::from("::1"));

    let some_num = Option::Some(100);
    let some_str = Option::Some("Hello");

    let none_value: Option<i32> = Option::None;

    println!("{:?}", some_num);

    match none_value {
        Option::Some(100) => println!("it is 100"),
        Option::None => println!("None"),
        _ => println!("Not found!"),
    }

    let value = 10 * coin_value(Coin::Dime);

    println!("10 dimes equal to {value} cents");

    let five = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(Option::None);

    println!("{:?} {:?}", &six, &none);
}

fn coin_value(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}
