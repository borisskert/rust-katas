#[cfg(test)]
mod tests {
    use rust_katas::help_the_bookseller::stock_list;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_test_1() {
        let b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");
    }

    #[test]
    fn basic_test_2() {
        let b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");
    }

    #[test]
    fn empty_tests() {
        let b = vec![];
        let c = vec!["A", "B", "C", "D"];
        dotest(b, c, "");
    }
}
