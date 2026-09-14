//! Utility for splitting long messages into smaller chunks.

/// Maximum length of a Telegram message in characters (4096).
pub const MAX_MESSAGE_LENGTH: usize = 4096;

/// Splits a long message into chunks of at most `max_len` UTF-8 characters.
///
/// It splits greedily on newline (`\n`) boundaries first, then on whitespace,
/// and only splits inside words if a single word exceeds `max_len`.
///
/// # Examples
/// ```
/// use ronami::utils::split_message;
///
/// let text = "hello\nworld\nfoo bar";
/// let chunks = split_message(text, 10);
/// assert_eq!(chunks, vec!["hello", "world", "foo bar"]);
/// ```
pub fn split_message(text: &str, max_len: usize) -> Vec<&str> {
    let max_len = if max_len == 0 { MAX_MESSAGE_LENGTH } else { max_len };

    if text.chars().count() <= max_len {
        return vec![text];
    }

    let mut chunks = Vec::new();
    let mut remaining = text;

    while !remaining.is_empty() {
        let char_count = remaining.chars().count();
        if char_count <= max_len {
            chunks.push(remaining);
            break;
        }

        let max_byte_idx =
            remaining.char_indices().nth(max_len).map(|(idx, _)| idx).unwrap_or(remaining.len());

        let slice = &remaining[..max_byte_idx];

        if let Some(pos) = slice.rfind('\n') {
            chunks.push(&remaining[..pos]);
            remaining = remaining[pos + 1..].trim_start_matches('\n');
        } else if let Some(pos) = slice.rfind(char::is_whitespace) {
            chunks.push(&remaining[..pos]);
            remaining = remaining[pos + 1..].trim_start_matches(char::is_whitespace);
        } else {
            chunks.push(slice);
            remaining = &remaining[max_byte_idx..];
        }
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_short_message() {
        let text = "short message";
        assert_eq!(split_message(text, 50), vec!["short message"]);
    }

    #[test]
    fn test_split_on_newline() {
        let text = "line 1\nline 2\nline 3";
        let chunks = split_message(text, 10);
        assert_eq!(chunks, vec!["line 1", "line 2", "line 3"]);
    }

    #[test]
    fn test_split_on_whitespace() {
        let text = "hello world foo bar";
        let chunks = split_message(text, 12);
        assert_eq!(chunks, vec!["hello world", "foo bar"]);
    }

    #[test]
    fn test_split_hard_boundary() {
        let text = "abcdefghijklmnop";
        let chunks = split_message(text, 5);
        assert_eq!(chunks, vec!["abcde", "fghij", "klmno", "p"]);
    }
}
