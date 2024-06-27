#[cfg(test)]
mod tests {
    use rust_katas::tic_tac_toe_checker::is_solved;

    fn dotest(board: &[&[u8; 3]; 3], expected: i8) {
        let actual = is_solved(board);
        assert_eq!(actual, expected, "With board = {board:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        for (board, expected) in [
            ([&[0, 0, 1], &[0, 1, 2], &[2, 1, 0]], -1),
            ([&[1, 1, 1], &[0, 2, 2], &[0, 0, 0]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 1, 2]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 2, 1]], 0)
        ] {
            dotest(&board, expected);
        }
    }

    #[test]
    fn failing_test() {
        dotest(&[&[2, 0, 2], &[2, 1, 1], &[1, 2, 1]], -1);
    }
    
    /*
    With board = [[0, 2, 1], [0, 1, 0], [1, 2, 0]]
Expected 1 but got -1 at src/lib.rs:116:9
     */
    
    #[test]
    fn failing_test2() {
        dotest(&[&[0, 2, 1], &[0, 1, 0], &[1, 2, 0]], 1);
    }
}
