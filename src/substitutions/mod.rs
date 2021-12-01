use std::ops::RangeInclusive;

use crate::clean::Clean;

pub type SubstitutionsList = Vec<(RangeInclusive<char>, &'static str)>;

macro_rules! substitutions {
    ($($($substitution_list:expr),+ => $subst:expr),*  $(,)?) => {{
        let mut x : Vec<(RangeInclusive<char>, &'static str)> = Vec::new();
        $(
            $(
                if let Some(f) = (&$substitution_list as &dyn std::any::Any).downcast_ref::<char>() {
                    x.push((*f ..=*f, $subst));
                }

                if let Some(f) = (&$substitution_list as &dyn std::any::Any).downcast_ref::<RangeInclusive<char>>() {
                    x.push((f.clone(), $subst));
                }
            )*
        )*

        x
    }};
}

pub(crate) use substitutions;

pub struct Substitute<'a> {
    substitutions_list: &'a SubstitutionsList,
    unicode_block_scope: &'a Option<String>,
}

/// Substitute a set of chars with a &str
impl<'a> Substitute<'a> {
    pub fn new(
        substitutions_list: &'a SubstitutionsList,
        unicode_block_scope: &'a Option<String>,
    ) -> Self {
        Self {
            substitutions_list,
            unicode_block_scope,
        }
    }
}

impl<'a> Clean for Substitute<'a> {
    type Data = String;
    fn clean(&self, data: &mut Self::Data) {
        for (ranges, substitution) in self.substitutions_list.clone() {
            for elem in ranges {
                let mut last_index = data.len();
                while let Some(i) = data[..last_index].rfind(elem) {
                    data.replace_range(i..(i + elem.len_utf8()), &substitution); //replace_range = no allocation
                    last_index = i;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitutions_list_generation() {
        let s1 = substitutions!(
            'à' => "a",
        );

        let expected_1 = vec![('à'..='à', "a")];

        assert_eq!(s1, expected_1);

        let s2 = substitutions!(
            'à' => "a",
            'a' ..= 'z' => "",
        );

        let expected_2 = vec![('à'..='à', "a"), ('a'..='z', "")];

        assert_eq!(s2, expected_2);
    }

    #[test]
    fn test_substitute() {
        let substitutions = substitutions!(
            'œ' => "oe",
            'æ' => "ae",
        );

        let mut input = "æ œ".to_string();
        Substitute::new(&substitutions, &None).clean(&mut input);

        assert_eq!("ae oe", input);
    }
}
