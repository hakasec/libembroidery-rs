use std::u8;
use std::str::FromStr;
use std::convert::{From, Into};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct EmbColour {
    r: u8,
    g: u8,
    b: u8,
}

pub type EmbColor = EmbColour;

impl From<ffi::EmbColor> for EmbColour {
    fn from(raw: ffi::EmbColor) -> EmbColour {
        EmbColour {
            r: raw.r as u8,
            g: raw.g as u8,
            b: raw.b as u8,
        }
    }
}

impl<'a> From<&'a str> for EmbColour {
    fn from(hex: &'a str) -> Self {
        EmbColour::from_str(hex).unwrap()
    }
}

impl Into<Box<str>> for EmbColour {
    fn into(self) -> Box<str> {
        format!("{:X}{:X}{:X}", self.r, self.g, self.b).into_boxed_str()
    }
}

impl FromStr for EmbColour {
    type Err = ParseIntError;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        let r = &hex[..2];
        let g = &hex[2..4];
        let b = &hex[4..];

        let rp = match u8::from_str_radix(r, 16) {
            Err(e) => return Err(e),
            Ok(n) => n,
        };
        let gp = match u8::from_str_radix(g, 16) {
            Err(e) => return Err(e),
            Ok(n) => n,
        };
        let bp = match u8::from_str_radix(b, 16) {
            Err(e) => return Err(e),
            Ok(n) => n,
        };

        Ok(EmbColour {
            r: rp,
            g: gp,
            b: bp,
        })
    }
}

impl EmbColour {
    pub fn new(r: u8, g: u8, b: u8) -> EmbColour {
        EmbColour { r, g, b }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_hex() {
        let c: EmbColour = EmbColour::from_str("00A1FF").unwrap();
        assert_eq!(c.r, 0);
        assert_eq!(c.g, 161);
        assert_eq!(c.b, 255);
    }

    #[test]
    fn test_from_hex_invalid() {
        assert!(EmbColour::from_str("GGGGGG").is_err());
    }

    #[test]
    fn test_into_hex() {
        let c: EmbColour = EmbColour::new(145, 56, 254);
        let s: Box<str> = c.into();
        assert_eq!(s.into_string(), "9138FE");
    }
}