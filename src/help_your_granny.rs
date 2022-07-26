use std::collections::HashMap;

/// https://www.codewars.com/kata/5536a85b6ed4ee5a78000035/train/rust
pub fn tour(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>) -> i32 {
    let distances = frnds.iter()
        .map(|friend| fr_twns.get(friend))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|town| dist.get(town))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<&f64>>();

    let grannies_distance = grannies_dist(distances);

    return grannies_distance as i32;
}

fn grannies_dist(distances: Vec<&f64>) -> f64 {
    let mut grannies_distance: f64 = 0.0;
    let mut x0: Option<f64> = None;

    for x in distances {
        if x0.is_none() {
            x0 = Some(*x);
            grannies_distance = *x;
        } else {
            let x0_unwrapped = x0.unwrap();
            grannies_distance += (x * x - x0_unwrapped * x0_unwrapped).sqrt();
            x0 = Some(*x);
        }
    }

    grannies_distance += x0.unwrap_or(0.0);

    grannies_distance
}
