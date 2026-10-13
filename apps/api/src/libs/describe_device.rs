use woothee::parser::Parser;

const UNKNOWN: &str = "UNKNOWN";

pub fn describe_device(user_agent: &str) -> String {
    let Some(result) = Parser::new().parse(user_agent) else {
        return "unknown".to_string();
    };

    let known = |value: &str| (!value.is_empty() && value != UNKNOWN).then(|| value.to_string());
    let with_version = |name: &str, version: &str| {
        known(name).map(|name| match known(version) {
            Some(version) => format!("{name} {version}"),
            None => name,
        })
    };

    let parts: Vec<String> = [
        known(result.category),
        with_version(result.os, &result.os_version),
        with_version(result.name, result.version),
    ]
    .into_iter()
    .flatten()
    .collect();

    if parts.is_empty() {
        "unknown".to_string()
    } else {
        parts.join(" · ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn devices_are_described_from_the_user_agent() {
        let iphone = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1";

        assert_eq!(
            describe_device(iphone),
            "smartphone · iPhone 17.5 · Safari 17.5"
        );
        assert_eq!(describe_device(""), "unknown");
    }
}
