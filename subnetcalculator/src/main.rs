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

    let ip = match parse_ip(given_ip.trim()) {
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

    let prefix = match parse_prefix(given_prefix.trim()) {
        Ok(prefix) => prefix,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    find_ip_data(&ip, &prefix);
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

fn find_ip_data(ip: &Ipv4Addr, prefix: &u8) {

    //Constant Values for the basic numbers
    let max_prefix: u8 = 32;
    let custom_hostbits: u32 = u32::from(max_prefix - prefix);

    //Subnetmask in bit
    let mask_in_bit: u32 = if custom_hostbits == 32 {
        0
    } else {
        u32::MAX << custom_hostbits
    };

    //Subnetmask as IP
    let subnetmask: Ipv4Addr = Ipv4Addr::from(mask_in_bit);
    
    let broadcast: Ipv4Addr = find_broadcast(&ip, &mask_in_bit);
    let netid: Ipv4Addr = find_netid(&ip, &mask_in_bit);

    println!("The subnetmask is: {subnetmask}");
    println!("The Broadcast is: {broadcast} and the NetID is: {netid}");
}

fn find_broadcast(ip: &Ipv4Addr, subnet_bit: &u32) -> Ipv4Addr {
    //Try to catch special-IPs
    if ip.is_broadcast() || ip.is_unspecified() {
        return *ip;
    } 
    //Calculate the broadcast
    else {
        //Broadcast-IP in bit
        let broadcast_in_bit = u32::from(*ip) | !subnet_bit;

        //Broadcast as IP
        let broadcast_ip: Ipv4Addr = Ipv4Addr::from(broadcast_in_bit);
        return broadcast_ip; 
    }
}

fn find_netid(ip: &Ipv4Addr, subnet_bit: &u32) -> Ipv4Addr {
    //NetID-IP in Bit
    let netid: u32 = u32::from(*ip) & subnet_bit;

    //NetID-IP as IP
    let netid_ip: Ipv4Addr = Ipv4Addr::from(netid);
    return netid_ip;
}
