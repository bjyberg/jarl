pub(crate) mod equal_assignment;

#[cfg(test)]
mod tests {
    use crate::utils_test::*;

    #[test]
    fn test_lint_equal_assignment() {
        use insta::assert_snapshot;
        let (lint_output, fix_output) = get_lint_and_fix_text(
            vec![
                "blah=1",
                "blah = 1",
                "blah = fun(1)",
                "fun((blah = fun(1)))",
                "1 -> fun",
                // TODO
                // "blah = fun(1) {",
            ],
            "equal_assignment",
        );
        assert_snapshot!("lint_output", lint_output);
        assert_snapshot!("fix_output", fix_output);
    }

    #[test]
    fn test_no_lint_equal_assignment() {
        assert!(no_lint("y <- 1", "equal_assignment",));
        assert!(no_lint("fun(y = 1)", "equal_assignment",));
        assert!(no_lint("y == 1", "equal_assignment",));
    }
}
