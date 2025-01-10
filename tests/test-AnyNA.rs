mod common;
use common::*;

#[test]
fn test_lint_any_na() {
    use insta::assert_snapshot;
    let temp_file = write_to_tempfile("any(is.na(x))\n");
    let lint_output = get_lint_text(&temp_file);
    let fix_output = get_fixed_text(&temp_file);
    assert_snapshot!("lint_output", lint_output);
    assert_snapshot!("fix_output", fix_output);
}

#[test]
fn test_no_lint_any_na() {
    assert!(no_lint("y <- any(x)"));
    assert!(no_lint("y <- is.na(x)"))
}
