use std::io;
use std::net::Ipv4Addr;
use std::str::FromStr;


fn main() {
    println!("Type some IP and subnetmask here!");

    let mut given_ip = String::new();

    io::stdin()
        .read_line(&mut given_ip)
        .expect("Failed to read the IP");

    match parse_ip(given_ip.trim()) {
        Ok(ip) => println!("Valid IP: {ip}!"),
        Err(err) => println!("{err}"),
    }
}

//Test if the ip is valid
fn parse_ip(input: &str) -> Result<Ipv4Addr, std::net::AddrParseError> {
    Ipv4Addr::from_str(input)
}
