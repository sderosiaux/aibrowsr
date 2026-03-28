/// Truncate a string to at most `max_chars` characters, appending `suffix` if truncated.
/// Safe on all UTF-8 strings — never panics on multi-byte characters.
pub fn truncate_str(s: &str, max_chars: usize, suffix: &str) -> String {
    let char_count = s.chars().count();
    if char_count <= max_chars {
        return s.to_string();
    }
    let truncated: String = s.chars().take(max_chars).collect();
    format!("{truncated}{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug_truncate_ascii() {
        assert_eq!(truncate_str("hello world", 5, "..."), "hello...");
        assert_eq!(truncate_str("short", 10, "..."), "short");
    }

    #[test]
    fn bug_truncate_utf8_multibyte() {
        // Each Japanese char is 3 bytes. Byte slicing at arbitrary positions panics.
        let japanese = "日本語テストデータ";
        let result = truncate_str(japanese, 3, "...");
        assert_eq!(result, "日本語...");
    }

    #[test]
    fn bug_truncate_mixed_utf8() {
        let mixed = "café résumé über";
        let result = truncate_str(mixed, 6, "...");
        assert_eq!(result, "café r...");
    }

    #[test]
    fn bug_truncate_emoji() {
        let emoji = "Hello 🌍🌍🌍 World";
        let result = truncate_str(emoji, 8, "...");
        assert_eq!(result, "Hello 🌍🌍...");
    }

    #[test]
    fn bug_truncate_zero() {
        assert_eq!(truncate_str("hello", 0, "..."), "...");
    }

    #[test]
    fn bug_truncate_empty() {
        assert_eq!(truncate_str("", 5, "..."), "");
    }
}
