#[cfg(test)]
mod tests {
    use rust_katas::josephus_survivor::josephus_survivor;

    #[test]
    fn basic_tests() {
        assert_eq!(josephus_survivor(37, 2), 11);
        assert_eq!(josephus_survivor(7, 3), 4);
        assert_eq!(josephus_survivor(11, 19), 10);
        assert_eq!(josephus_survivor(40, 3), 28);
        assert_eq!(josephus_survivor(14, 2), 13);
        assert_eq!(josephus_survivor(100, 1), 100);
        assert_eq!(josephus_survivor(1, 300), 1);
        assert_eq!(josephus_survivor(2, 300), 1);
        assert_eq!(josephus_survivor(5, 300), 1);
        assert_eq!(josephus_survivor(7, 300), 7);
        assert_eq!(josephus_survivor(300, 300), 265);
    }
}
