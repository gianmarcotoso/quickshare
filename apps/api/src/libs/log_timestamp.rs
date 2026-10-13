use chrono::{DateTime, Local, Utc};

pub fn log_timestamp(moment: DateTime<Utc>) -> String {
    format!(
        "{} ({})",
        moment.format("%Y-%m-%dT%H:%M:%SZ"),
        moment.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S %:z")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_are_in_utc_followed_by_local_time_with_offset() {
        let moment = DateTime::parse_from_rfc3339("2026-10-13T17:01:22Z")
            .unwrap()
            .with_timezone(&Utc);
        let timestamp = log_timestamp(moment);

        assert!(
            timestamp.starts_with("2026-10-13T17:01:22Z ("),
            "{timestamp}"
        );
        assert!(timestamp.ends_with(')'));
        assert!(timestamp.contains(':'));
    }
}
