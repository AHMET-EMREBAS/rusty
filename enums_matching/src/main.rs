enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    println!("Hello, world!");

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddrKind::V6(String::from("::1"));
}
