use std::net::Ipv4Addr;
use clap::{Parser, value_parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///IP-Adress of subnet
    #[arg(short, long)]
    ip: Ipv4Addr,

    ///Prefix of subnetmask
    #[arg(short, long, value_parser = value_parser!(u32).range(0..=32),
    default_value_t = 24)]
    prefix: u32,
}


fn main() {
    let args = Args::parse();

    find_ip_data(&args.ip, &args.prefix);
}

fn find_ip_data(ip: &Ipv4Addr, prefix: &u32) {

    //Constant Values for the basic numbers
    let max_prefix: u32 = 32;
    let custom_hostbits: u32 = max_prefix - prefix;

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
