use std::net::IpAddr;

use crate::libs::http_url::http_url;

const VIRTUAL_INTERFACE_PREFIXES: &[&str] = &["docker", "br-", "veth", "virbr", "vmnet", "vboxnet"];

pub struct NetworkUrl {
    pub interface: String,
    pub url: String,
}

pub fn network_urls(bind: &str, port: u16) -> Vec<NetworkUrl> {
    let Ok(address) = bind.parse::<IpAddr>() else {
        return vec![NetworkUrl {
            interface: String::new(),
            url: format!("http://{bind}:{port}/"),
        }];
    };

    if !address.is_unspecified() {
        return vec![NetworkUrl {
            interface: String::new(),
            url: http_url(address, port),
        }];
    }

    let mut urls: Vec<NetworkUrl> = local_ip_address::list_afinet_netifas()
        .unwrap_or_default()
        .into_iter()
        .filter(|(interface, address)| {
            address.is_ipv4()
                && !address.is_loopback()
                && !VIRTUAL_INTERFACE_PREFIXES
                    .iter()
                    .any(|prefix| interface.starts_with(prefix))
        })
        .map(|(interface, address)| NetworkUrl {
            interface,
            url: http_url(address, port),
        })
        .collect();

    urls.sort_by(|a, b| a.interface.cmp(&b.interface).then(a.url.cmp(&b.url)));
    urls.dedup_by(|a, b| a.url == b.url);

    urls
}
