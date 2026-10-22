pub fn numbered_file_name(name: &str, number: u32) -> String {
    if number == 0 {
        return name.to_string();
    }

    match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => format!("{stem} ({number}).{extension}"),
        _ => format!("{name} ({number})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicates_are_numbered_before_the_extension() {
        assert_eq!(numbered_file_name("report.pdf", 0), "report.pdf");
        assert_eq!(numbered_file_name("report.pdf", 2), "report (2).pdf");
        assert_eq!(
            numbered_file_name("archive.tar.gz", 1),
            "archive.tar (1).gz"
        );
        assert_eq!(numbered_file_name(".env", 1), ".env (1)");
        assert_eq!(numbered_file_name("README", 1), "README (1)");
    }
}
