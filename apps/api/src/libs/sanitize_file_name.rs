const FALLBACK_NAME: &str = "file";
const MAX_NAME_BYTES: usize = 200;
const RESERVED_CHARACTERS: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

pub fn sanitize_file_name(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or_default();

    let cleaned: String = base
        .chars()
        .map(|c| {
            if c.is_control() || RESERVED_CHARACTERS.contains(&c) {
                '_'
            } else {
                c
            }
        })
        .collect();

    let trimmed = cleaned.trim().trim_matches('.').trim();

    if trimmed.is_empty() {
        return FALLBACK_NAME.to_string();
    }

    truncate(trimmed, MAX_NAME_BYTES)
}

fn truncate(name: &str, max_bytes: usize) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_and_reserved_characters_are_removed() {
        assert_eq!(sanitize_file_name("report.pdf"), "report.pdf");
        assert_eq!(sanitize_file_name("../../etc/passwd"), "passwd");
        assert_eq!(sanitize_file_name("C:\\Users\\me\\photo.jpg"), "photo.jpg");
        assert_eq!(sanitize_file_name("a:b*c?.txt"), "a_b_c_.txt");
        assert_eq!(sanitize_file_name("line\nbreak.txt"), "line_break.txt");
    }

    #[test]
    fn empty_or_dot_names_get_a_fallback() {
        assert_eq!(sanitize_file_name(""), "file");
        assert_eq!(sanitize_file_name(".."), "file");
        assert_eq!(sanitize_file_name("dir/"), "file");
    }

    #[test]
    fn long_names_keep_their_extension() {
        let name = format!("{}.pdf", "è".repeat(150));
        let sanitized = sanitize_file_name(&name);

        assert!(sanitized.len() <= MAX_NAME_BYTES);
        assert!(sanitized.ends_with(".pdf"));
    }
}
