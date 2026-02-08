use crate::diagnostic::*;
use crate::utils::{get_arg_by_name_then_position, get_arg_by_position, node_contains_comments};
use air_r_syntax::*;
use biome_rowan::AstNode;

pub struct ExpectMatch;

/// ## What it does
///
/// Checks for usage of `expect_true(grepl(...))`.
///
/// Placeholder for the `expect_match` rule implementation.
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

pub fn expect_match(_ast: &RCall) -> anyhow::Result<Option<Diagnostic>> {
    Ok(None)
}
