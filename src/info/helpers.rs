/// Strip ANSI escape codes from a string and return clean text.
pub fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if chars.peek() == Some(&'[') {
                chars.next();
                // Skip until we hit a letter
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Extract version number from a version string line.
/// Looks for parts starting with a digit or 'v'.
#[allow(dead_code)]
pub fn parse_version_from_line(line: &str) -> Option<&str> {
    line.split_whitespace().find(|part| {
        part.chars()
            .next()
            .map(|c| c.is_ascii_digit() || c == 'v')
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_codes() {
        assert_eq!(strip_ansi_codes("hello"), "hello");
        assert_eq!(strip_ansi_codes("\x1b[31mred\x1b[0m"), "red");
        assert_eq!(
            strip_ansi_codes("\x1b[1;32mbold green\x1b[0m"),
            "bold green"
        );
    }

    #[test]
    fn test_parse_version_from_line() {
        assert_eq!(parse_version_from_line("zsh 5.9"), Some("5.9"));
        assert_eq!(parse_version_from_line("v1.2.3"), Some("v1.2.3"));
        assert_eq!(parse_version_from_line("hello world"), None);
        assert_eq!(parse_version_from_line("program 1.0.0"), Some("1.0.0"));
    }
}
