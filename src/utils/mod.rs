pub fn get_previous_char(text: &str, pos: usize) -> Option<char> {
    if pos == 0 {
        return None;
    }

    text.chars().nth(pos - 1)
}

pub fn get_next_char(text: &str, pos: usize) -> Option<char> {
    if pos == text.len() {
        return None;
    }

    text.chars().nth(pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_previous_char() {
        assert_eq!(get_previous_char("abc", 0), None);
        assert_eq!(get_previous_char("abc", 1), Some('a'));
        assert_eq!(get_previous_char("abc", 2), Some('b'));
        assert_eq!(get_previous_char("abc", 3), Some('c'));
        assert_eq!(get_previous_char("abc", 4), None);
        assert_eq!(get_previous_char("œabc", 1), Some('œ'));
        assert_eq!(get_previous_char("aœbc", 1), Some('a'));
    }
}
