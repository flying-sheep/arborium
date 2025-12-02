//! Comprehensive injection tests for languages with embedded code.
//!
//! These tests verify that language injections (e.g., CSS in HTML `<style>` tags)
//! work correctly by recording highlight events and asserting against them.

#[cfg(test)]
mod tests {
    use crate::highlighter::Highlighter;
    use crate::tree_sitter_highlight::{HighlightEvent, Highlight};
    use crate::tree_sitter_highlight::Highlighter as TsHighlighter;
    use crate::HIGHLIGHT_NAMES;
    use indoc::indoc;

    /// A recorded highlight event for testing
    #[derive(Debug, Clone, PartialEq)]
    enum Event {
        /// Source text was emitted
        Source { text: String },
        /// Highlight started with this name
        Start { name: String },
        /// Highlight ended
        End,
    }

    /// Record all highlight events for a given language and source
    fn record_events(language: &str, source: &str) -> Vec<Event> {
        let mut highlighter = Highlighter::new();
        let normalized = normalize_lang(language);

        let config = highlighter.configs.get(normalized)
            .expect(&format!("Language {} not found", language));

        let names: Vec<String> = HIGHLIGHT_NAMES.iter().map(|s| s.to_string()).collect();

        let mut ts_highlighter = TsHighlighter::new();
        let highlights = ts_highlighter
            .highlight(config, source.as_bytes(), None, |lang| highlighter.configs.get(lang))
            .expect("Failed to highlight");

        let mut events = Vec::new();
        for event in highlights {
            let event = event.expect("Highlight event error");
            match event {
                HighlightEvent::Source { start, end } => {
                    events.push(Event::Source {
                        text: source[start..end].to_string()
                    });
                }
                HighlightEvent::HighlightStart(Highlight(i)) => {
                    let name = if i < HIGHLIGHT_NAMES.len() {
                        HIGHLIGHT_NAMES[i].to_string()
                    } else {
                        format!("unknown_{}", i)
                    };
                    events.push(Event::Start { name });
                }
                HighlightEvent::HighlightEnd => {
                    events.push(Event::End);
                }
            }
        }
        events
    }

    fn normalize_lang(language: &str) -> &str {
        match language {
            "js" | "jsx" => "javascript",
            "ts" => "typescript",
            _ => language,
        }
    }

    /// Check that specific highlight names appear in the events
    fn assert_has_highlights(events: &[Event], expected_names: &[&str], context: &str) {
        let found_names: std::collections::HashSet<_> = events.iter()
            .filter_map(|e| match e {
                Event::Start { name } => Some(name.as_str()),
                _ => None,
            })
            .collect();

        for expected in expected_names {
            assert!(
                found_names.contains(expected),
                "{}: Expected highlight '{}' not found. Found: {:?}",
                context,
                expected,
                found_names
            );
        }
    }

    /// Check that a specific text appears with a specific highlight
    fn assert_text_highlighted(events: &[Event], text: &str, highlight: &str, context: &str) {
        let mut current_highlights: Vec<&str> = Vec::new();
        let mut found = false;

        for event in events {
            match event {
                Event::Start { name } => {
                    current_highlights.push(name);
                }
                Event::End => {
                    current_highlights.pop();
                }
                Event::Source { text: src } => {
                    if src.contains(text) && current_highlights.iter().any(|h| *h == highlight) {
                        found = true;
                        break;
                    }
                }
            }
        }

        assert!(
            found,
            "{}: Text '{}' should be highlighted as '{}'. Events: {:?}",
            context, text, highlight, events
        );
    }

    // ========================================================================
    // HTML Tests
    // ========================================================================

    #[test]
    #[cfg(all(feature = "lang-html", feature = "lang-css", feature = "lang-javascript"))]
    fn test_html_isolated_style() {
        let source = indoc! {r#"
            <style>
                h1 { color: red; }
            </style>
        "#};
        let events = record_events("html", source);

        // Should have CSS property highlighting
        assert_has_highlights(&events, &["property"], "HTML style injection");
    }

    #[test]
    #[cfg(all(feature = "lang-html", feature = "lang-css", feature = "lang-javascript"))]
    fn test_html_isolated_script() {
        let source = indoc! {r#"
            <script>
                let x = 1;
                const y = "hello";
            </script>
        "#};
        let events = record_events("html", source);

        // Should have JS keyword highlighting
        assert_has_highlights(&events, &["keyword"], "HTML script injection");
        assert_text_highlighted(&events, "let", "keyword", "HTML script injection");
    }

    #[test]
    #[cfg(all(feature = "lang-html", feature = "lang-css", feature = "lang-javascript"))]
    fn test_html_mixed_content() {
        let source = indoc! {r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body { margin: 0; }
                </style>
            </head>
            <body>
                <h1>Hello</h1>
                <script>
                    console.log("world");
                </script>
            </body>
            </html>
        "#};
        let events = record_events("html", source);

        // Should have both CSS and JS highlighting
        assert_has_highlights(&events, &["tag", "property", "string"], "HTML mixed content");
    }

    // ========================================================================
    // Svelte Tests
    // ========================================================================

    #[test]
    #[cfg(all(feature = "lang-svelte", feature = "lang-css", feature = "lang-javascript"))]
    fn test_svelte_isolated_script() {
        let source = indoc! {r#"
            <script>
                let name = "world";
                export let count = 0;
            </script>
        "#};
        let events = record_events("svelte", source);

        // Should have JS keyword highlighting
        assert_has_highlights(&events, &["keyword"], "Svelte script injection");
        assert_text_highlighted(&events, "let", "keyword", "Svelte script injection");
    }

    #[test]
    #[cfg(all(feature = "lang-svelte", feature = "lang-css", feature = "lang-javascript"))]
    fn test_svelte_isolated_style() {
        let source = indoc! {r#"
            <style>
                h1 {
                    color: red;
                    font-size: 2em;
                }
            </style>
        "#};
        let events = record_events("svelte", source);

        // Should have CSS property highlighting
        assert_has_highlights(&events, &["property"], "Svelte style injection");
    }

    #[test]
    #[cfg(all(feature = "lang-svelte", feature = "lang-css", feature = "lang-javascript"))]
    fn test_svelte_template_expressions() {
        let source = indoc! {r#"
            <h1>Hello {name}!</h1>
            <p>Count: {count + 1}</p>
        "#};
        let events = record_events("svelte", source);

        // Template expressions should be highlighted
        // The {name} and {count + 1} should have some highlighting
        assert!(!events.is_empty(), "Svelte template should produce events");
    }

    #[test]
    #[cfg(all(feature = "lang-svelte", feature = "lang-css", feature = "lang-javascript"))]
    fn test_svelte_full_component() {
        let source = indoc! {r#"
            <script>
                export let name = "world";
                let count = 0;

                function increment() {
                    count += 1;
                }
            </script>

            <main>
                <h1>Hello {name}!</h1>
                <button on:click={increment}>
                    Clicked {count} times
                </button>
            </main>

            <style>
                main {
                    text-align: center;
                    padding: 1em;
                }

                h1 {
                    color: #ff3e00;
                }

                button {
                    background: #ff3e00;
                    color: white;
                }
            </style>
        "#};
        let events = record_events("svelte", source);

        // Should have JS keywords
        assert_has_highlights(&events, &["keyword"], "Svelte full component - JS");
        assert_text_highlighted(&events, "export", "keyword", "Svelte full component");
        assert_text_highlighted(&events, "function", "keyword", "Svelte full component");

        // Should have CSS properties
        assert_has_highlights(&events, &["property"], "Svelte full component - CSS");
    }

    #[test]
    #[cfg(all(feature = "lang-svelte", feature = "lang-css", feature = "lang-javascript", feature = "lang-typescript"))]
    fn test_svelte_typescript() {
        let source = indoc! {r#"
            <script lang="ts">
                interface User {
                    name: string;
                    age: number;
                }

                let user: User = { name: "Alice", age: 30 };
            </script>
        "#};
        let events = record_events("svelte", source);

        // Should have TypeScript highlighting
        assert_has_highlights(&events, &["keyword"], "Svelte TypeScript");
    }

    // ========================================================================
    // Vue Tests
    // ========================================================================

    #[test]
    #[cfg(all(feature = "lang-vue", feature = "lang-css", feature = "lang-javascript"))]
    fn test_vue_isolated_script() {
        let source = indoc! {r#"
            <script>
            export default {
                data() {
                    return { name: "world" };
                }
            }
            </script>
        "#};
        let events = record_events("vue", source);

        // Should have JS keyword highlighting
        assert_has_highlights(&events, &["keyword"], "Vue script injection");
        assert_text_highlighted(&events, "export", "keyword", "Vue script injection");
    }

    #[test]
    #[cfg(all(feature = "lang-vue", feature = "lang-css", feature = "lang-javascript"))]
    fn test_vue_isolated_style() {
        let source = indoc! {r#"
            <style>
            .hello {
                color: blue;
                font-weight: bold;
            }
            </style>
        "#};
        let events = record_events("vue", source);

        // Should have CSS property highlighting
        assert_has_highlights(&events, &["property"], "Vue style injection");
    }

    #[test]
    #[cfg(all(feature = "lang-vue", feature = "lang-css", feature = "lang-javascript"))]
    fn test_vue_scoped_style() {
        let source = indoc! {r#"
            <style scoped>
            .hello {
                color: red;
            }
            </style>
        "#};
        let events = record_events("vue", source);

        // Should have CSS property highlighting even with scoped attribute
        assert_has_highlights(&events, &["property"], "Vue scoped style injection");
    }

    #[test]
    #[cfg(all(feature = "lang-vue", feature = "lang-css", feature = "lang-javascript"))]
    fn test_vue_full_sfc() {
        let source = indoc! {r#"
            <template>
                <div class="hello">
                    <h1>{{ msg }}</h1>
                </div>
            </template>

            <script>
            export default {
                name: 'HelloWorld',
                props: {
                    msg: String
                }
            }
            </script>

            <style scoped>
            .hello {
                text-align: center;
            }
            h1 {
                font-weight: normal;
            }
            </style>
        "#};
        let events = record_events("vue", source);

        // Should have JS keywords
        assert_has_highlights(&events, &["keyword"], "Vue SFC - JS");

        // Should have CSS properties
        assert_has_highlights(&events, &["property"], "Vue SFC - CSS");
    }

    #[test]
    #[cfg(all(feature = "lang-vue", feature = "lang-css", feature = "lang-javascript", feature = "lang-typescript"))]
    fn test_vue_typescript() {
        let source = indoc! {r#"
            <script lang="ts">
            import { defineComponent } from 'vue';

            interface Props {
                msg: string;
            }

            export default defineComponent({
                props: {
                    msg: String
                }
            });
            </script>
        "#};
        let events = record_events("vue", source);

        // Should have TypeScript highlighting
        assert_has_highlights(&events, &["keyword"], "Vue TypeScript");
    }

    // ========================================================================
    // Edge Cases and Regression Tests
    // ========================================================================

    #[test]
    #[cfg(all(feature = "lang-html", feature = "lang-css", feature = "lang-javascript"))]
    fn test_empty_style_tag() {
        let source = "<style></style>";
        let events = record_events("html", source);
        // Should not crash
        assert!(!events.is_empty());
    }

    #[test]
    #[cfg(all(feature = "lang-html", feature = "lang-css", feature = "lang-javascript"))]
    fn test_empty_script_tag() {
        let source = "<script></script>";
        let events = record_events("html", source);
        // Should not crash
        assert!(!events.is_empty());
    }

    #[test]
    #[cfg(all(feature = "lang-svelte", feature = "lang-css", feature = "lang-javascript"))]
    fn test_svelte_only_template() {
        let source = indoc! {r#"
            <div>
                <h1>Hello World</h1>
                <p>No script or style tags here</p>
            </div>
        "#};
        let events = record_events("svelte", source);
        // Should not crash and should have some content
        assert!(!events.is_empty());
    }

    #[test]
    #[cfg(all(feature = "lang-svelte", feature = "lang-css", feature = "lang-javascript"))]
    fn test_svelte_nested_braces() {
        let source = indoc! {r#"
            <script>
                let obj = { a: { b: { c: 1 } } };
            </script>
        "#};
        let events = record_events("svelte", source);
        assert_has_highlights(&events, &["keyword"], "Svelte nested braces");
    }

    #[test]
    #[cfg(all(feature = "lang-html", feature = "lang-css", feature = "lang-javascript"))]
    fn test_html_inline_event_handler() {
        let source = r#"<button onclick="alert('hello')">Click</button>"#;
        let events = record_events("html", source);
        // Should handle inline handlers
        assert!(!events.is_empty());
    }

    // ========================================================================
    // High-Level API Tests
    // ========================================================================

    #[test]
    #[cfg(all(feature = "lang-svelte", feature = "lang-css", feature = "lang-javascript"))]
    fn test_highlighter_api_svelte() {
        let mut highlighter = Highlighter::new();
        let source = indoc! {r#"
            <script>
                let x = 1;
            </script>
            <style>
                h1 { color: red; }
            </style>
        "#};

        let html = highlighter.highlight_to_html("svelte", source).unwrap();

        // JS should be highlighted
        assert!(html.contains("<a-k>let</a-k>"),
            "JS keyword should be highlighted. Got: {}", html);

        // CSS should have highlighting tags
        assert!(html.contains("<a-"),
            "CSS should have highlighting. Got: {}", html);
    }

    #[test]
    #[cfg(all(feature = "lang-vue", feature = "lang-css", feature = "lang-javascript"))]
    fn test_highlighter_api_vue() {
        let mut highlighter = Highlighter::new();
        let source = indoc! {r#"
            <script>
            export default {
                data() { return {}; }
            }
            </script>
            <style>
            .foo { color: blue; }
            </style>
        "#};

        let html = highlighter.highlight_to_html("vue", source).unwrap();

        // JS should be highlighted
        assert!(html.contains("<a-k>export</a-k>"),
            "JS keyword should be highlighted. Got: {}", html);
    }

    #[test]
    #[cfg(all(feature = "lang-html", feature = "lang-css", feature = "lang-javascript"))]
    fn test_highlighter_api_html() {
        let mut highlighter = Highlighter::new();
        let source = indoc! {r#"
            <script>
                const greeting = "hello";
            </script>
            <style>
                body { margin: 0; }
            </style>
        "#};

        let html = highlighter.highlight_to_html("html", source).unwrap();

        // JS should be highlighted
        assert!(html.contains("<a-k>const</a-k>"),
            "JS keyword should be highlighted. Got: {}", html);
    }
}
