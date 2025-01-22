use regex::Regex;
use std::fmt;

/// https://www.codewars.com/kata/54a91a4883a7de5d7800009c/train/rust
pub fn increment_string(s: &str) -> String {
    Incrementable::new(s).increment().to_string()
}

struct Incrementable {
    prefix: String,
    digits: Digits,
}

impl Incrementable {
    fn new(s: &str) -> Incrementable {
        let pattern = Regex::new(r"(.*[a-zA-Z])?([0-9]+)?$").unwrap();

        let Some(caps) = pattern.captures(s) else {
            return Incrementable {
                prefix: s.to_string(),
                digits: Digits::zero(),
            };
        };

        let prefix = caps.get(1).map_or("", |m| m.as_str());
        let digits = caps
            .get(2)
            .map_or(Digits::zero(), |m| Digits::from_str(m.as_str()));

        Incrementable {
            prefix: prefix.to_string(),
            digits,
        }
    }

    fn increment(&self) -> Incrementable {
        let digits = self.digits.increment();

        Incrementable {
            prefix: self.prefix.clone(),
            digits,
        }
    }
}

impl fmt::Display for Incrementable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.prefix, self.digits)
    }
}

struct Digits {
    digits: String,
}

impl Digits {
    fn zero() -> Digits {
        Digits {
            digits: "0".to_string(),
        }
    }

    fn one() -> Digits {
        Digits {
            digits: "1".to_string(),
        }
    }

    fn from_str(s: &str) -> Digits {
        let digits = s
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .collect::<String>();

        Digits { digits }
    }

    fn increment(&self) -> Digits {
        if self.digits.is_empty() {
            return Digits::one();
        }

        let nines = self.digits.chars().rev().take_while(|c| c == &'9').count();

        if nines > 0 {
            let prefix = &self.digits[..self.digits.len() - nines];
            let digits = format!(
                "{}{}",
                Digits::from_str(prefix).increment(),
                "0".repeat(nines)
            );

            return Digits { digits };
        }

        let last = self.digits.chars().last().unwrap();

        if last.is_ascii_digit() {
            let prefix = &self.digits[..self.digits.len() - 1];
            let last = last as u8 - b'0' + 1;
            let digits = format!("{}{}", prefix, last);

            return Digits { digits };
        }

        Digits::one()
    }
}

impl fmt::Display for Digits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.digits)
    }
}
