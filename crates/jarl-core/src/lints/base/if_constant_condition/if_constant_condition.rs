use crate::diagnostic::*;
use crate::utils::{get_arg_by_name_then_position, get_arg_by_position, node_contains_comments};
use air_r_syntax::*;
use biome_rowan::AstNode;


/// ## What it does
///
/// Detects `if` conditions that are always `TRUE` or `FALSE`
///
/// ## Why is this bad?
///
/// Code with constant conditions will either never run or always run.
/// It clutters the code and makes it more difficult to read. 
///
/// ## Example
///
/// ```r
/// if (TRUE) {
///   print("always true")
/// }
///
/// if (FALSE && ...) {
///   print("always false")
/// }
///
/// if (TRUE || ...) {
///   print("always true")
/// }
/// ```
