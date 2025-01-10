/// https://www.codewars.com/kata/550f22f4d758534c1100025a/train/rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    pub fn is_opposite(&self, other: Direction) -> bool {
        match self {
            Direction::North => other == Direction::South,
            Direction::South => other == Direction::North,
            Direction::East => other == Direction::West,
            Direction::West => other == Direction::East,
        }
    }
}

pub fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut result: Vec<Direction> = Vec::new();

    for &direction in arr {
        if let Some(last) = result.last() {
            if last.is_opposite(direction) {
                result.pop();
            } else {
                result.push(direction);
            }
        } else {
            result.push(direction);
        }
    }

    result
}
