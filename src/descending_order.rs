use itertools::Itertools;

// https://www.codewars.com/kata/5467e4d82edf8bbf40000155/train/rust
pub fn descending_order(x: u64) -> u64 {
    Digits::new(x).sorted().reversed().to_number()
}

struct Digits {
    digits: Vec<u64>,
}

impl Digits {
    fn new(x: u64) -> Self {
        Digits {
            digits: to_digits(x),
        }
    }

    fn sorted(&self) -> Self {
        Digits {
            digits: self.digits.iter().sorted().copied().collect(),
        }
    }

    fn reversed(&self) -> Self {
        Digits {
            digits: self.digits.iter().rev().copied().collect(),
        }
    }

    fn to_number(&self) -> u64 {
        from_digits(self.digits.clone())
    }
}

fn to_digits(mut x: u64) -> Vec<u64> {
    let mut digits = Vec::new();

    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }

    digits
}

fn from_digits(digits: Vec<u64>) -> u64 {
    digits.iter().fold(0, |acc, &digit| acc * 10 + digit)
}
