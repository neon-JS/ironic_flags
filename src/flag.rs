use regex::Regex;
use crate::rgb;
use serde::{Serialize, Deserialize};

/// Represents a flag
///
/// # Examples
/// ```
/// let german_flag = Flag {
///     name: String::from("Deutschland"),
///     parts: vec!(black_part, red_part, gold_part)
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    pub parts: Vec<FlagPart>,
    pub name: String,
}

/// Represents a part of a flag (e.g. bar)
///
/// # Examples
/// ```
/// let red_bar = FlagPart {
///     color: String::from("#ff0000"),
///     relation: 1
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct FlagPart {
    /// RGB color of FlagPart
    /// Must contain six hexadecimal characters and _can_ contain a hash.
    pub color: String,
    /// Width of part
    /// Percentage will be calculated by the relation to the sum of all parts
    /// (equals CSS' `flex-grow`).
    pub relation: usize,
}

impl FlagPart {
    /// Returns color code of a FlagParts' color
    pub fn color_string(&self) -> String
    {
        let regex = Regex::new(r"^#?([0-9a-fA-F]{6})$").unwrap();

        let value = regex.captures(&self.color)
            .and_then(|captures|captures.get(1))
            .map(|v| v.as_str())
            .map(|v| i32::from_str_radix(v, 16))
            .expect("Color is not of correct format (#RRGGBB)!")
            .unwrap();

        let r = (value >> 16 & 0xFF) as u8;
        let g = (value >> 8 & 0xFF) as u8;
        let b = (value >> 0 & 0xFF) as u8;

        rgb(r, g, b)
    }
}