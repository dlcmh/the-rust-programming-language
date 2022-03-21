use std::net::IpAddr;

fn main() {
    // Use `unwrap` if it's certain that `Result` will always be `Ok`;
    // in this case, it's clear that 127.0.0.1 is a valid IP address that can be parsed,
    // and that the `Err` variant will never be returned.
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{}", home); // 127.0.0.1
}
