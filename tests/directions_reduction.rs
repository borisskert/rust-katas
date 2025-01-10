use rust_katas::directions_reduction::*;

#[cfg(test)]
mod tests {
    use super::{dir_reduc, Direction::{*}};

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }
}
