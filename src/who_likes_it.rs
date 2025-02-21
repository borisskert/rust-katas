use itertools::Itertools;

// https://www.codewars.com/kata/5266876b8f4bf2da9b000362/train/rust
pub fn likes(names: &[&str]) -> String {
    let count = names.len();

    match count {
        0 => "no one likes this".to_string(),
        1 => names.first().unwrap().to_string() + " likes this",
        2 => revaluate(names) + " like this",
        3 => revaluate(names) + " like this",
        _ => {
            names.iter().take(2).join(", ")
                + " and "
                + &format!("{} others", count - 2)
                + " like this"
        }
    }
}

fn revaluate(names: &[&str]) -> String {
    let count = names.len();

    names.iter().take(count - 1).join(", ") + " and " + names.last().unwrap()
}

// #againwhatlearned
// Use destructuring:
// match names {
//   [] => format!("no one likes this"),
//   [a] => format!("{} likes this", a),
//   [a, b] => format!("{} and {} like this", a, b),
//   [a, b, c] => format!("{}, {} and {} like this", a, b, c),
//   [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
// }
