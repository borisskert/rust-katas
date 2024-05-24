///
/// https://www.codewars.com/kata/52774a314c2333f0a7000688/train/rust
/// # Arguments
///
/// * `s`: String containing parentheses
///
/// returns: bool if the order of the parentheses is valid
///
pub fn valid_parentheses(s: &str) -> bool {
    s.chars()
        .fold(0, next_balance) == 0
}

fn next_balance(balance: i8, c: char) -> i8 {
    if balance < 0 {
        return balance;
    }

    if c == '(' {
        return balance + 1;
    } else if c == ')' {
        return balance - 1;
    }

    balance
}

// #againwhatlearned
// Use pattern matching instead of if-else:
// return match c {
//  '(' => balance + 1,
//  ')' => balance - 1,
//  _ => balance
// };
