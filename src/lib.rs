#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::doc_markdown)]
#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/azzamsa/dryip-of-rust/master/docs/logo.png
"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/azzamsa/dryip-of-rust/master/docs/favicon.ico
"
)]

//! # Dryip of Rust 🦀
//!
//! Hello, and welcome to the Dryip of Rust 🦀 website! 👋
//!
//! - Each function categorized into appropriate module.
//! - Use the search bar to find the function.
//! - Click the `[SRC]` button at the top to see the complete source code.
//!

/// Array snippets.
pub mod arrays;

/// Date snippets.
pub mod dates;

/// Math snippets.
pub mod maths;

/// String snippets.
pub mod strings;

/// Sort the slice (non mutating).
///
/// Rust currently doesn't have a non-mutating sort function
/// <https://github.com/rust-lang/rfcs/issues/2731>.
/// There is a handy third-party tool [itertools](https://docs.rs/itertools/0.10.1/itertools/fn.sorted.html)
/// But we minimize the use of any external dependencies, as the project's goal is learning.
#[must_use]
pub fn sorted(word: &str) -> String {
    let mut chars_ = word.chars().collect::<Vec<char>>();
    chars_.sort_unstable();
    chars_.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted() {
        assert_eq!("aaagmnr", sorted("anagram"));
        assert_eq!("aceimn", sorted("iceman"));
    }
}
