//! TYPE grammar for tree-sitter
//!
//! This crate provides the type language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_type() -> Language;
}

/// Returns the type tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_type() }
}

/// The highlights query for type (empty - no highlights available).
pub const HIGHLIGHTS_QUERY: &str = "";

/// The injections query for type (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for type (empty - no locals available).
pub const LOCALS_QUERY: &str = "";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        arborium_test_harness::test_grammar(
            language(),
            "type",
            HIGHLIGHTS_QUERY,
            INJECTIONS_QUERY,
            LOCALS_QUERY,
            env!("CARGO_MANIFEST_DIR"),
        );
    }
}
