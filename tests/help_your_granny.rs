#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rust_katas::help_your_granny::tour;

    fn testing(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>, exp: i32) -> () {
        assert_eq!(tour(&frnds, fr_twns, dist), exp)
    }

    macro_rules! hash_map (
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

    #[test]
    fn tests_tour() {
        let friends = ["A1", "A2", "A3", "A4", "A5"];
        let fr_towns = hash_map! { "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4" };
        let dst = hash_map! { "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0 };
        testing(&friends, fr_towns, dst, 889);
    }

    #[test]
    fn test_empty() {
        let friends = [];
        let fr_towns = hash_map! { "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4" };
        let dst = hash_map! { "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0 };
        testing(&friends, fr_towns, dst, 0);
    }
}
