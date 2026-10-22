pub fn truncate_file_name(name: &str, max_bytes: usize) -> String {
    if name.len() <= max_bytes {
        return name.to_string();
    }

    let (stem, extension) = match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() && extension.len() < max_bytes / 2 => {
            (stem, format!(".{extension}"))
        }
        _ => (name, String::new()),
    };

    let mut end = max_bytes - extension.len();
    while !stem.is_char_boundary(end) {
        end -= 1;
    }

    format!("{}{extension}", &stem[..end])
}
