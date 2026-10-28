use std::io;
use std::net::Ipv4Addr;
use std::str::FromStr;
use ipnet::{IpNet, Ipv4Net};


fn main() {
    println!("Type some IP and subnetmask here!");

    let mut given_ip = String::new();
    let mut given_prefix = String::new();

    io::stdin()
        .read_line(&mut given_ip)
        .expect("Failed to read the IP");

    match parse_ip(given_ip.trim()) {
        Ok(ip) => println!("Valid IP: {ip}!"),
        Err(err) => println!("{err}"),
    }

    match parse_prefix(given_prefix.trim()) {
        Ok(prefix) => println!("Valid subnetmask: {prefix}!"),
        Err(err) => println!({err}),
    }
}

//Test if the ip is valid
fn parse_ip(input: &str) -> Result<Ipv4Addr, std::net::AddrParseError> {
    Ipv4Addr::from_str(input)
}

//Test if the subnetmask is valid
fn parse_prefix(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str(input)
}
