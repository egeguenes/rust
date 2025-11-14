use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    mode::{Port, Subdomain}
};
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};
use rayon::prelude::*;

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    let sokcet_addrs: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socker_addrs.len() == 0 {
        return subdomain;
    }

    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_iter()
        .map(|ports| scan_port(socker_addrs[0], *port))
        .filter(|port| port.is_open)
        .collect();
    subdomain
}

fn scan_port(mut socket_addr: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    socket_addr.set_port(port);

    let is_open = if let Ok(_) = TcpStream::connect_timeout(&socket_addr, timeout) {
        true
    } else {
        false
    };

    Port {
        port: port,
        is_open,
    }
}
