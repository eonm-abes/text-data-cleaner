use std::ops::RangeInclusive;


use crate::clean::Clean;
use crate::common::{AppendAfterCharIfDifferent, PrependBeforeCharIfDifferent, AppendAfterCharIf, PrependBeforeCharIf};
use crate::whitespaces::SpaceTrimmer;
use crate::normalize::Normalize;
use crate::substitutions::{substitutions, Substitute, SubstitutionsList};

/// Substitutions for the French language
pub struct FrenchSubstitutions {
    substitutions_list: SubstitutionsList,
    unicode_block_scope: Option<String>,
}

impl FrenchSubstitutions {
    pub fn new() -> FrenchSubstitutions {
        let substitutions_list = substitutions! {
            'œ' => "oe",
            'Œ' => "OE",
            'æ' => "ae",
            'Æ' => "AE",
            '`' => "'",
            '’' => "'",
            '“' => "\"",
            '”' => "\"",
            '‘' => "'",
            '’' => "'",
            '«' => "\"",
            '»' => "\"",
        };

        FrenchSubstitutions {
            substitutions_list,
            unicode_block_scope: None,
        }
    }
}

impl<'a> From<&'a FrenchSubstitutions> for Substitute<'a> {
    fn from(substitutions: &'a FrenchSubstitutions) -> Self {
        Substitute::new(
            &substitutions.substitutions_list,
            &substitutions.unicode_block_scope,
        )
    }
}

impl Clean for FrenchSubstitutions {
    type Data = String;

    fn clean(&self, data: &mut Self::Data) {
        let substitueur: Substitute = self.into();
        substitueur.clean(data);
    }
}

/// Normalize for the French language
pub struct FrenchTypography;

impl FrenchTypography {
    pub fn new() -> FrenchTypography {
        FrenchTypography
    }
}

impl Normalize for FrenchTypography {
    type Data = String;

    fn normalize(&self, data: &mut Self::Data) {
        let char_starting_with_space = ['»', '(', ':', ';', '!', '?', '«'];
        let char_ending_with_space = ['.', '«', ',', ')', ':', ';'];


        let test_next_char_is_space_or_punct = move |next_char: Option<char>, matching_char: char, _replacement: char| {
            let french_punct = ['.'];
            
            if let Some(next_char) = next_char {
                println!("matching char `{}` next `{}`", matching_char, next_char,);
                !next_char.is_whitespace() && !french_punct.contains(&next_char)
            } else {
                true
            }
        };
        
               
        for elem in char_starting_with_space {
            PrependBeforeCharIfDifferent::new(elem, ' ').normalize(data);
        }
        
        for elem in char_ending_with_space {
            AppendAfterCharIf::new(elem, ' ', test_next_char_is_space_or_punct).normalize(data);
        }

        SpaceTrimmer::new().clean(data);
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_french_typography() {
        let mut input = "«Je suis un texte: en français.(e).»".to_string();
        let expected = "« Je suis un texte : en français. (e). »".to_string();

        FrenchTypography::new().normalize(&mut input);

        assert_eq!(input, expected);
    }
}
