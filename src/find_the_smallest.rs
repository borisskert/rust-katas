/// https://www.codewars.com/kata/573992c724fc289553000e95/train/rust
pub fn smallest(n: i64) -> (i64, usize, usize) {
    Result::potential_results(n)
        .iter()
        .min_by_key(|result| result.value)
        .map(Result::to_tuple)
        .unwrap()
}

struct Result {
    value: i64,
    i: usize,
    j: usize,
}

impl Result {
    fn potential_results(n: i64) -> Vec<Result> {
        let len = length(n);

        cartesian_product(len, len)
            .iter()
            .map(|(i, j)| {
                let value = move_digit(n, *i, *j);

                Result {
                    value,
                    i: *i,
                    j: *j,
                }
            })
            .collect()
    }

    fn to_tuple(&self) -> (i64, usize, usize) {
        (self.value, self.i, self.j)
    }
}

fn digits(n: i64) -> Vec<i8> {
    let mut digits = Vec::new();
    let mut i = n;

    while i > 0 {
        digits.push((i % 10) as i8);
        i /= 10;
    }

    digits.reverse();
    digits
}

fn from_digits(digits: &[i8]) -> i64 {
    digits.iter()
        .fold(0, |acc, &digit| acc * 10 + (digit as i64))
}

fn length(n: i64) -> usize {
    n.to_string().len()
}

fn move_digit(n: i64, i: usize, j: usize) -> i64 {
    if i == j {
        return n;
    }

    let mut digits = digits(n);
    let digit = digits.remove(i);

    digits.insert(j, digit);

    from_digits(&digits)
}

fn cartesian_product(a: usize, b: usize) -> Vec<(usize, usize)> {
    (0..a)
        .flat_map(|x| (0..b)
            .map(move |y| (x, y))
        )
        .collect()
}
