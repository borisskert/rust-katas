#[cfg(test)]
mod tests {
    use rust_katas::string_incrementer::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert_eq!(
            actual, expected,
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\""
        )
    }

    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
        dotest("foobar00999", "foobar01000");
        dotest(
            "HereComesATrickyTest99999999999999999999999999999999999999",
            "HereComesATrickyTest100000000000000000000000000000000000000",
        );
        dotest(
            "HereCome9TrickyTests99999999999999999999999999999999999999",
            "HereCome9TrickyTests100000000000000000000000000000000000000",
        );
        dotest("joZrZTOCZoGLNvNXTJXi1LmWNBUQQFuKTfpy8LnmSxkxOB47QJRPYf4Rui2Ip9FmSprAW5OUIHTRBI7znHv5guUszVl4jMJ273943689171169197935269935199995483298998809461999159939949959069565904356979419556691", "joZrZTOCZoGLNvNXTJXi1LmWNBUQQFuKTfpy8LnmSxkxOB47QJRPYf4Rui2Ip9FmSprAW5OUIHTRBI7znHv5guUszVl4jMJ273943689171169197935269935199995483298998809461999159939949959069565904356979419556692");
    }
}
