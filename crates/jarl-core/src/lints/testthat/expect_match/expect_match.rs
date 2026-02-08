use crate::diagnostic::*;
use crate::utils::{get_arg_by_name_then_position, get_function_name};
use air_r_syntax::*;
use biome_rowan::AstNode;

pub struct ExpectMatch;

/// ## What it does
///
/// Checks for usage of `expect_true(grepl(...))`.
///
/// ## Why is this bad?
///
/// `expect_match()` is more explicit and clearer in intent than wrapping
/// `grepl()` in `expect_true()`. It also provides better error messages when
/// tests fail.
///
/// This rule is **disabled by default**. Select it either with the rule name
/// `"expect_match"` or with the rule group `"TESTTHAT"`.
///
/// ## Example
///
/// ```r
/// expect_true(grepl("foo", x))
/// expect_true(base::grepl("bar", x))
/// ```
///
/// Use instead:
/// ```r
/// expect_match(x, "foo")
/// expect_match(x, "bar")
/// ```
impl Violation for ExpectMatch {
    fn name(&self) -> String {
        "expect_match".to_string()
    }

    fn body(&self) -> String {
        "`expect_true(grepl(...))` is not as clear as expect_match(...).".to_string()
    }

    fn suggestion(&self) -> Option<String> {
        Some("Use `expect_match(...)` instead.".to_string())
    }
}

pub fn expect_match(ast: &RCall) -> anyhow::Result<Option<Diagnostic>> {
    let function = ast.function()?;
    let function_name = get_function_name(function);
    if function_name != "expect_true" {
        return Ok(None);
    }

    let args = ast.arguments()?.items();
    let object = unwrap_or_return_none!(get_arg_by_name_then_position(&args, "object", 1));
    let object_value = unwrap_or_return_none!(object.value());

    let grepl_call = unwrap_or_return_none!(object_value.as_r_call());
    let grepl_function = grepl_call.function()?;
    let grepl_name = get_function_name(grepl_function);
    if grepl_name != "grepl" {
        return Ok(None);
    }

    let grepl_args = grepl_call.arguments()?.items();
    let pattern_arg = get_arg_by_name_then_position(&grepl_args, "pattern", 1);
    let x_arg = get_arg_by_name_then_position(&grepl_args, "x", 2);

    if pattern_arg.is_none() || x_arg.is_none() {
        return Ok(None);
    }

    let range = ast.syntax().text_trimmed_range();
    let diagnostic = Diagnostic::new(ExpectMatch, range, Fix::empty());

    Ok(Some(diagnostic))
}
