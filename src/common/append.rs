use crate::normalize::Normalize;

/// Append a character after a character in a String.
pub struct AppendAfterChar {
    target_char: char,
    char_to_insert: char,
}

impl AppendAfterChar {
    pub fn new(target_char: char, char_to_insert: char) -> Self {
        Self {
            target_char,
            char_to_insert,
        }
    }
}

impl Normalize for AppendAfterChar {
    type Data = String;

    fn normalize(&self, data: &mut Self::Data) {
        data.rfind(self.target_char)
            .map(|index| data.insert(index + self.target_char.len_utf8(), self.char_to_insert));
    }
}

pub struct AppendAfterCharIf {
    target_char: char,
    char_to_insert: char,
    condition: fn(Option<char>, char, char) -> bool,
}

impl AppendAfterCharIf {
    pub fn new(
        target_char: char,
        char_to_insert: char,
        condition: fn(Option<char>, char, char) -> bool,
    ) -> Self {
        Self {
            target_char,
            char_to_insert,
            condition,
        }
    }
}

impl Normalize for AppendAfterCharIf {
    type Data = String;

    fn normalize(&self, data: &mut Self::Data) {
        data.rfind(self.target_char).map(|index| {
            let next_char_index = index + self.target_char.len_utf8();
            
            let next_char = if next_char_index <= data.len() {
                data[index + self.target_char.len_utf8()..].chars().nth(0)
            } else {
                None
            };

            if (self.condition)(next_char, self.target_char, self.char_to_insert) {
                data.insert(index + self.target_char.len_utf8(), self.char_to_insert);
            }
        });
    }
}

/// Append a character after a character if the target char does not already exist.
#[derive(Clone, Copy)]
pub struct AppendAfterCharIfDifferent {
    target_char: char,
    char_to_insert: char,
}

impl AppendAfterCharIfDifferent {
    pub fn new(target_char: char, char_to_insert: char) -> Self {
        Self {
            target_char,
            char_to_insert,
        }
    }
}

impl From<AppendAfterCharIfDifferent> for AppendAfterCharIf {
    fn from(other: AppendAfterCharIfDifferent) -> Self {
        let func = |next_char: Option<char>, _matching_char: char, replacement: char| {
            if let Some(next_char) = next_char {
                next_char != replacement
            } else {
                true
            }
        };

        Self::new(other.target_char, other.char_to_insert, func)
    }
}

impl Normalize for AppendAfterCharIfDifferent {
    type Data = String;

    fn normalize(&self, data: &mut Self::Data) {
        AppendAfterCharIf::from(*self).normalize(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut data = String::from("abc");

        let append = AppendAfterChar::new('b', 'd');
        append.normalize(&mut data);

        assert_eq!(data, "abdc");

        let mut data = String::from("abœc");
        let append = AppendAfterChar::new('œ', 'd');

        append.normalize(&mut data);
        assert_eq!(data, "abœdc");
    }

    #[test]
    fn test_should_not_append_if() {
        let mut data = String::from("; ");

        let func = |next_char: Option<char>, _matching_char: char, replacement: char| {
            if let Some(next_char) = next_char {
                next_char != replacement
            } else {
                true
            }
        };

        let append_if = AppendAfterCharIf::new(';', ' ', func);
        append_if.normalize(&mut data);

        assert_eq!("; ", data);

        let mut data = String::from(" : ee");

        let func = |next_char: Option<char>, _matching_char: char, replacement: char| {
            if let Some(next_char) = next_char {
                next_char != replacement
            } else {
                true
            }
        };

        let append_if = AppendAfterCharIf::new(':', ' ', func);
        append_if.normalize(&mut data);

        assert_eq!(" : ee", data);
    }

    #[test]
    fn test_should_append_if() {
        let mut data = String::from(";");

        let func = |next_char: Option<char>, _matching_char: char, replacement: char| {
            if let Some(next_char) = next_char {
                next_char != replacement
            } else {
                true
            }
        };

        let append_if = AppendAfterCharIf::new(';', ' ', func);
        append_if.normalize(&mut data);

        assert_eq!("; ", data);

        let mut data = String::from(":eee");

        let func = |next_char: Option<char>, _matching_char: char, replacement: char| {
            if let Some(next_char) = next_char {
                !next_char.is_ascii_whitespace()
            } else {
                true
            }
        };

        let append_if = AppendAfterCharIf::new(':', ' ', func);
        append_if.normalize(&mut data);

        assert_eq!(": eee", data);
    }

    #[test]
    fn test_should_append_if_different() {
        let mut data = String::from(";");

        let append_if_different = AppendAfterCharIfDifferent::new(';', ' ');
        append_if_different.normalize(&mut data);

        assert_eq!("; ", data);
    }

    #[test]
    fn test_should_not_append_if_different() {
        let mut data = String::from("; ");

        let append_if_different = AppendAfterCharIfDifferent::new(';', ' ');
        append_if_different.normalize(&mut data);

        assert_eq!("; ", data);

        let mut data = String::from("; e");

        let append_if_different = AppendAfterCharIfDifferent::new(';', ' ');
        append_if_different.normalize(&mut data);

        assert_eq!("; e", data);

        let mut data = String::from(" : eee");

        let append_if_different = AppendAfterCharIfDifferent::new(':', ' ');
        append_if_different.normalize(&mut data);

        assert_eq!(" : eee", data);

        
    }
}
