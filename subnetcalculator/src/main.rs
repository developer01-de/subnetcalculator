use std::{error, io};
use std::net::Ipv4Addr;
use std::str::FromStr;
use ipnet::{IpNet, Ipv4Net, Ipv4Subnets};


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

fn find_broadcast(ip: &Ipv4Addr, prefix: u8) -> Ipv4Addr {

    //Constant Values for the basic numbers
    let max_prefix: u32 = 32;
    let status_of_bit: u32 = 2;

    //Try to catch special-IPs
    if ip.is_broadcast() || ip.is_unspecified() {
        let special_ip: Ipv4Addr = *ip;
        return special_ip;
    } 
    //Calculate the broadcast
    else {
        let custom_bits: u32 = max_prefix - u32::from(prefix);
        let ips_per_subnet: u32 = status_of_bit.pow(custom_bits);

    }
}
