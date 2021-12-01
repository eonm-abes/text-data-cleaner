use std::ops::RangeInclusive;
use unicode_normalization::UnicodeNormalization;

mod whitespaces;
use whitespaces::*;

mod common;
use common::*;

mod utils;
use utils::*;

mod clean;
mod normalize;

use clean::*;
use normalize::Normalize;

mod french;
use french::FrenchSubstitutions;

mod substitutions;
use substitutions::{substitutions, Substitute};

// substituer les exposants !
// substituer subscript

fn main() {
    let input = &mut "œﬃ «’eee\teee\t\t   eee";
    let mut x = input.nfkc().collect::<String>();

    FrenchSubstitutions::new().clean(&mut x);
    ControlCharRemover.clean(&mut x);
    ConsecutiveWhiteSpaceRemover.clean(&mut x);
    SpaceNormalizer.normalize(&mut x);

    println!("{}", x);
}

/// Removes all unicode control characters
pub struct ControlCharRemover;

impl Clean for ControlCharRemover {
    type Data = String;

    fn clean(&self, data: &mut Self::Data) {
        data.retain(|c| !c.is_control());
    }
}

/// Normalizes all unicode characters to their canonical decomposition
pub struct NfkcNormalizer;

impl Normalize for NfkcNormalizer {
    type Data = String;

    fn normalize(&self, data: &mut Self::Data) {
        *data = data.nfkc().collect::<String>();
    }
}

// pub struct Substitute {
//     substitutions: vec![(vec![char], char)]
// }

// impl Substitute {
//     pub fn new() -> Self {
//         Substitute { substitutions: vec![] }
//     }

//     pub fn add(&mut self, from: Vec<char>, to: char) -> &mut Self {
//         self.substitutions.push((from, to));
//         self
//     }

//     pub fn append(&mut self, substitutions: vec![(vec![char], char)]) -> &mut Self {
//         self.substitutions.append(&mut substitutions);
//         self
//     }
// }
