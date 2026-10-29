use std::{error, io};
use std::net::Ipv4Addr;
use std::str::FromStr;
use ipnet::{IpNet, Ipv4Net};


fn main() {
    println!("Type some IP here:");

    let mut given_ip = String::new();
    let mut given_prefix = String::new();

    io::stdin()
        .read_line(&mut given_ip)
        .expect("Failed to read the IP");

    let ipaddress = match parse_ip(given_ip.trim()) {
        Ok(ip) => ip,
        Err(err) => {
            println!("{err}");
            return;
        }
    };


    println!("Type some prefix here:");
    io::stdin()
        .read_line(&mut given_prefix)
        .expect("Failed to read prefix");

    match parse_prefix(given_prefix.trim()) {
        Ok(prefix) => println!("Valid subnetmask: {prefix}!"),
        Err(err) => println!("{err}"),
    }
}

//Test if the ip is valid
fn parse_ip(input: &str) -> Result<Ipv4Addr, std::net::AddrParseError> {
    Ipv4Addr::from_str(input)
}

//Test if the subnetmask is valid
fn parse_prefix(input: &str) -> Result<u8, String> {
    let prefix = input
        .parse::<u8>()
        .map_err(|err| err.to_string())?;

    if prefix <= 32 {
        Ok(prefix)
    } else {
        Err("The prefix has to be smaller than 32".to_string())
    }
}
