use crate::normalize::Normalize;
use crate::utils::get_previous_char;

/// Prepend a character before a character in a String.
pub struct PrependBeforeChar {
    target_char: char,
    char_to_insert: char,
}

impl PrependBeforeChar {
    pub fn new(target_char: char, char_to_insert: char) -> Self {
        Self {
            target_char,
            char_to_insert,
        }
    }
}

impl Normalize for PrependBeforeChar {
    type Data = String;

    fn normalize(&self, data: &mut Self::Data) {
        data.rfind(self.target_char)
            .map(|index| data.insert(index, self.char_to_insert));
    }
}

/// Prepend a character before a character in a String based on a condition.
pub struct PrependBeforeCharIf {
    target_char: char,
    char_to_insert: char,
    condition: fn(Option<char>, char, char) -> bool,
}

impl PrependBeforeCharIf {
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

impl Normalize for PrependBeforeCharIf {
    type Data = String;

    fn normalize(&self, data: &mut Self::Data) {
        data.rfind(self.target_char).map(|index| {
            let previous_char = &data[..index].chars().last();

            if (self.condition)(*previous_char, self.target_char, self.char_to_insert) {
                data.insert(index, self.char_to_insert);
            }
        });
    }
}

/// Prepend a character before a character if the target char does not already exist.
#[derive(Clone, Copy)]
pub struct PrependBeforeCharIfDifferent {
    target_char: char,
    char_to_insert: char,
}

impl PrependBeforeCharIfDifferent {
    pub fn new(target_char: char, char_to_insert: char) -> Self {
        Self {
            target_char,
            char_to_insert,
        }
    }
}

impl From<PrependBeforeCharIfDifferent> for PrependBeforeCharIf {
    fn from(other: PrependBeforeCharIfDifferent) -> Self {
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

impl Normalize for PrependBeforeCharIfDifferent {
    type Data = String;

    fn normalize(&self, data: &mut Self::Data) {
        PrependBeforeCharIf::from(*self).normalize(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend() {
        let mut data = String::from("abc");

        let prepend = PrependBeforeChar::new('b', 'd');
        prepend.normalize(&mut data);

        assert_eq!(data, "adbc");

        let mut data = String::from("abœc");
        let prepend = PrependBeforeChar::new('œ', 'd');

        prepend.normalize(&mut data);
        assert_eq!(data, "abdœc");
    }

    #[test]
    fn test_should_not_prepend_if() {
        let mut data = String::from(" ;");

        let func = |previous_char: Option<char>, _matching_char: char, replacement: char| {
            eprint!("previous_char {:?}", previous_char);
            if let Some(previous_char) = previous_char {
                previous_char != replacement
            } else {
                true
            }
        };

        let prepend_if = PrependBeforeCharIf::new(';', ' ', func);
        prepend_if.normalize(&mut data);

        assert_eq!(" ;", data);

        let mut data = String::from("œ;");

        let func = |previous_char: Option<char>, _matching_char: char, replacement: char| {
            eprint!("previous_char {:?}", previous_char);
            if let Some(previous_char) = previous_char {
                previous_char != replacement
            } else {
                true
            }
        };

        let prepend_if = PrependBeforeCharIf::new(';', 'œ', func);
        prepend_if.normalize(&mut data);

        assert_eq!("œ;", data);

        let mut data = String::from("lorœm");

        let func = |previous_char: Option<char>, _matching_char: char, replacement: char| {
            eprint!("previous_char {:?}", previous_char);
            if let Some(previous_char) = previous_char {
                previous_char != replacement
            } else {
                true
            }
        };

        let prepend_if = PrependBeforeCharIf::new('œ', 'œ', func);
        prepend_if.normalize(&mut data);

        assert_eq!("lorœœm", data);
    }

    #[test]
    fn test_should_prepend_if() {
        let mut data = String::from(";");

        let func = |next_char: Option<char>, _matching_char: char, replacement: char| {
            if let Some(next_char) = next_char {
                next_char != replacement
            } else {
                true
            }
        };

        let prepend_if = PrependBeforeCharIf::new(';', ' ', func);
        prepend_if.normalize(&mut data);

        assert_eq!(" ;", data);
    }

    #[test]
    fn test_should_prepend_if_different() {
        let mut data = String::from(";");

        let prepend_if_different = PrependBeforeCharIfDifferent::new(';', ' ');
        prepend_if_different.normalize(&mut data);

        assert_eq!(" ;", data);

        let mut data = String::from("lorem;");

        let prepend_if_different = PrependBeforeCharIfDifferent::new(';', ' ');
        prepend_if_different.normalize(&mut data);

        assert_eq!("lorem ;", data);
    }

    #[test]
    fn test_should_not_prepend_if_different() {
        let mut data = String::from(" ;");

        let prepend_if_different = PrependBeforeCharIfDifferent::new(';', ' ');
        prepend_if_different.normalize(&mut data);

        assert_eq!(" ;", data);
    }
}
