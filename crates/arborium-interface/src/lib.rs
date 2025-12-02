//! INTERFACE grammar for tree-sitter
//!
//! This crate provides the interface language grammar for use with tree-sitter.

use tree_sitter_patched_arborium::Language;

unsafe extern "C" {
    fn tree_sitter_interface() -> Language;
}

/// Returns the interface tree-sitter language.
pub fn language() -> Language {
    unsafe { tree_sitter_interface() }
}

/// The highlights query for interface (empty - no highlights available).
pub const HIGHLIGHTS_QUERY: &str = "";

/// The injections query for interface (empty - no injections available).
pub const INJECTIONS_QUERY: &str = "";

/// The locals query for interface (empty - no locals available).
pub const LOCALS_QUERY: &str = "";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        arborium_test_harness::test_grammar(
            language(),
            "interface",
            HIGHLIGHTS_QUERY,
            INJECTIONS_QUERY,
            LOCALS_QUERY,
            env!("CARGO_MANIFEST_DIR"),
        );
    }
}
