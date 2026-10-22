use crate::libs::truncate_file_name::truncate_file_name;

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

    truncate_file_name(trimmed, MAX_NAME_BYTES)
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
