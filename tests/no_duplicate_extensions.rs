use std::collections::HashSet;

use bat::assets::HighlightingAssets;

#[test]
fn no_duplicate_extensions() {
    const KNOWN_EXCEPTIONS: &[&'static str] = &[
        // The '.h' extension currently appears in multiple syntaxes: C, C++, Objective C,
        // Objective C++
        "h",
        // In addition to the standard Haskell syntax in 'Packages', we also ship the 'Cabal'
        // syntax which comes with a "Haskell (improved)" syntax.
        "hs",
        // In addition to the standard JavaScript syntax in 'Packages', we also ship the
        // 'Javascript (Babel)' syntax.
        "js",
        // The "Ruby Haml" syntax also comes with a '.sass' extension. However, we make sure
        // that 'sass' is mapped to the 'Sass' syntax.
        "sass",
    ];

    let assets = HighlightingAssets::from_binary();

    let mut extensions = HashSet::new();

    for syntax in assets.syntaxes() {
        for extension in &syntax.file_extensions {
            assert!(
                KNOWN_EXCEPTIONS.contains(&extension.as_str()) || extensions.insert(extension),
                "File extension / pattern \"{}\" appears twice in the syntax set",
                extension
            );
        }
    }
}
