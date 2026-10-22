use std::net::IpAddr;

pub fn http_url(address: IpAddr, port: u16) -> String {
    match address {
        IpAddr::V4(address) => format!("http://{address}:{port}/"),
        IpAddr::V6(address) => format!("http://[{address}]:{port}/"),
    }
}
