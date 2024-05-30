
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rust_katas::simple_assembler_interpreter::simple_assembler;

    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
             let mut map = HashMap::new();
             $(
                 map.insert($key.to_string(), $value);
             )*
             map
        }};
    }

    #[test]
    fn mov_instruction_test() {
        let program = vec!["mov a 5"];
        let expected = map! { "a" => 5 };
        compare_registers(expected, simple_assembler(program));
        
        let program = vec!["mov a 5", "mov b a"];
        let expected = map! { "a" => 5, "b" => 5 };
        compare_registers(expected, simple_assembler(program));
    }
    
    #[test]
    fn inc_instruction_test() {
        let program = vec!["mov a 5", "inc a"];
        let expected = map! { "a" => 6 };
        compare_registers(expected, simple_assembler(program));
        
        let program = vec!["mov a 5", "inc a", "inc a"];
        let expected = map! { "a" => 7 };
        compare_registers(expected, simple_assembler(program));
    }
    
    #[test]
    fn dec_instruction_test() {
        let program = vec!["mov a 5", "dec a"];
        let expected = map! { "a" => 4 };
        compare_registers(expected, simple_assembler(program));
        
        let program = vec!["mov a 5", "dec a", "dec a"];
        let expected = map! { "a" => 3 };
        compare_registers(expected, simple_assembler(program));
    }
    
    #[test]
    fn jnz_instruction_test() {
        let program = vec!["mov a 1", "jnz a 2", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));
        
        let program = vec!["mov a 0", "jnz a 2", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));
        
        let program = vec!["mov a 0", "jnz 0 2", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));
        
        let program = vec!["mov a 0", "jnz 1 2", "inc a"];
        let expected = map! { "a" => 0 };
        compare_registers(expected, simple_assembler(program));
    }
    
    #[test]
    fn short_tests() {
        let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));

        let program = vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ];
        let expected = map! { "a" => 409600, "c" => 409600, "b" => 409600};
        compare_registers(expected, simple_assembler(program));
    }

    fn compare_registers(expected: HashMap<String, i64>, actual: HashMap<String, i64>) {
        let result = expected
            .iter()
            .all(|(key, value)| actual.get(key).map(|v| v == value).unwrap_or(false));
        assert!(
            result,
            "Expected the registers to be like that:\n{:#?}\n\nBut got this:\n{:#?}\n",
            expected, actual
        )
    }
}
