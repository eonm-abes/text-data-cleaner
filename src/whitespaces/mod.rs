use crate::clean::Clean;
use crate::normalize::Normalize;

/// Replaces all kind of spaces (`\t`,`\n`, etc.) char with a standard space char ` ` (U+0020)
pub struct SpaceNormalizer;

impl Normalize for SpaceNormalizer {
    type Data = String;

    fn normalize(&self, data: &mut Self::Data) {
        let normal_space = " ";
        let mut last_index = data.len();
        while let Some(i) = data[..last_index].rfind(char::is_whitespace) {
            data.replace_range(i..(i + 1), &normal_space); //replace_range = no allocation
            last_index = i;
        }
    }
}

/// Removes all consecutive spaces with a single space
pub struct ConsecutiveWhiteSpaceRemover;

impl Clean for ConsecutiveWhiteSpaceRemover {
    type Data = String;

    fn clean(&self, data: &mut Self::Data) {
        let mut previous_char: Option<char> = None;

        //retain = no allocation
        data.retain(|c| {
            if let Some(p) = previous_char {
                previous_char = Some(c);

                !(p.is_ascii_whitespace() && c.is_ascii_whitespace())
            } else {
                previous_char = Some(c);
                true
            }
        });
    }
}

/// Removes leading and trailing spaces
pub struct SpaceTrimmer;

impl SpaceTrimmer {
    pub fn new() -> Self {
        SpaceTrimmer
    }
}


impl Clean for SpaceTrimmer {
    type Data = String;
    fn clean(&self, data: &mut Self::Data) {
        while data.ends_with(char::is_whitespace) {
            data.pop();
        }

        while data.starts_with(char::is_whitespace) {
            data.drain(..1);
        }
    }
}

/// Normalize spaces in a String
/// Applies the following transformations:
/// - Replaces all kind of spaces (`\t`,`\n`, etc.) char with a standard space char ` ` (U+0020)
/// - Removes all consecutive spaces with a single space
/// - Removes all leading and trailing spaces
pub struct WhitespaceNormalizer;

impl Clean for WhitespaceNormalizer {
    type Data = String;
    fn clean(&self, data: &mut Self::Data) {
        SpaceNormalizer.normalize(data);
        ConsecutiveWhiteSpaceRemover.clean(data);
        SpaceTrimmer.clean(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_spaces() {
        let input = " \tlorem ipsum dolor sit amet, consectetur adipiscing elit.\n ";
        let expected = "lorem ipsum dolor sit amet, consectetur adipiscing elit.";

        let mut data = input.to_string();
        SpaceTrimmer.clean(&mut data);

        assert_eq!(data, expected);
    }

    #[test]
    fn test_remove_consecutive_spaces() {
        let input = "lorem ipsum dolor sit\t\n amet, consectetur adipiscing elit.\n ";
        let expected = "lorem ipsum dolor sit\tamet, consectetur adipiscing elit.\n";

        let mut data = input.to_string();
        ConsecutiveWhiteSpaceRemover.clean(&mut data);

        assert_eq!(data, expected);
    }

    #[test]
    fn test_space_normalizer() {
        let input = "lorem ipsum dolor sit\t\n amet, consectetur adipiscing elit.\n ";
        let expected = "lorem ipsum dolor sit   amet, consectetur adipiscing elit.  ";

        let mut data = input.to_string();
        SpaceNormalizer.normalize(&mut data);

        assert_eq!(data, expected);
    }

    #[test]
    fn test_whitespace_normalizer() {
        let input = "lorem ipsum dolor sit\t\n amet, consectetur adipiscing elit.\n ";
        let expected = "lorem ipsum dolor sit amet, consectetur adipiscing elit.";

        let mut data = input.to_string();
        WhitespaceNormalizer.clean(&mut data);

        assert_eq!(data, expected);
    }
}
