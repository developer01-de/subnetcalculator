use std::io;
use std::net::Ipv4Addr;
use std::str::FromStr;


fn main() {
    println!("Type some IP here:");

    let mut given_ip = String::new();
    let mut given_prefix = String::new();

    io::stdin()
        .read_line(&mut given_ip)
        .expect("Failed to read the IP");

    match parse_ip(given_ip.trim()) {
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
        Ok(prefix) => println!("{prefix}"),
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    let broadcast_ip: Ipv4Addr = find_broadcast(given_ip.parse(), given_prefix.parse());
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

fn find_ip_data(ip_as_string: &str, prefix: u32) {

    let ip_address: Ipv4Addr = ip_as_string.parse();

    //Constant Values for the basic numbers
    let max_prefix: u32 = 32;
    let custom_hostbits: u32 = max_prefix - u32::from(prefix);
    
    

    find_broadcast(ip_address, custom_hostbits);
    find_netid(custom_hostbits);
}

fn find_broadcast(ip: &Ipv4Addr, hostbits: u32) -> Ipv4Addr {
    //Try to catch special-IPs
    if ip.is_broadcast() || ip.is_unspecified() {
        return *ip;
    } 
    //Calculate the broadcast
    else {
        let broadcast_in_bit: u32 = u32::MAX << hostbits;
        let broadcast_ip: Ipv4Addr = std::net::Ipv4Addr::from(broadcast_in_bit);
        return broadcast_ip; 
    }
}

fn find_netid(hostbits: u32) -> Ipv4Addr {
    let netid_in_bits: u32 = u32::MIN << hostbits;
    let netid_ip: Ipv4Addr = std::net::Ipv4Addr::from(netid_in_bits);
    return netid_ip;
}
