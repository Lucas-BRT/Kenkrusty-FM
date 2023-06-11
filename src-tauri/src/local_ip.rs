use std::net::IpAddr;

pub fn get_local_ips() -> Vec<(String, IpAddr)> {
    let addresses = local_ip_address::list_afinet_netifas().unwrap();
    let mut ipv4_addresses = Vec::new();

    for (string, ip) in addresses {
        if let IpAddr::V4(_) = ip {
            ipv4_addresses.push((string, ip));
        }
    }
    ipv4_addresses
}
