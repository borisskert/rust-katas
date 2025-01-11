/// https://www.codewars.com/kata/55c6126177c9441a570000cc/train/rust
pub fn order_weight(s: &str) -> String {
    Weights::of(s).sorted().as_string()
}

#[derive(Clone)]
struct Weight {
    s: String,
    value: u32,
}

impl Weight {
    fn cmp(&self, other: &Weight) -> std::cmp::Ordering {
        if self.value == other.value {
            return self.s.cmp(&other.s);
        }

        self.value.cmp(&other.value)
    }

    fn new(weight: &str) -> Weight {
        Weight {
            s: weight.to_string(),
            value: ascii_sum(weight),
        }
    }

    fn multiple_of(weight: &str) -> Vec<Weight> {
        weight.split_whitespace().map(|w| Weight::new(w)).collect()
    }
}

struct Weights {
    weights: Vec<Weight>,
}

impl Weights {
    fn of(s: &str) -> Weights {
        Weights {
            weights: Weight::multiple_of(s),
        }
    }

    fn sorted(&self) -> Weights {
        let mut weights = self.weights.clone();
        weights.sort_by(|a, b| a.cmp(b));

        Weights { weights }
    }

    fn as_string(&self) -> String {
        self.weights
            .iter()
            .map(|w| w.s.as_str())
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

fn ascii_sum(s: &str) -> u32 {
    s.chars().map(|c| c as u32 - '0' as u32).sum()
}
