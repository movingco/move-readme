# rustdoc-to-markdown

Transform code blocks from rustdoc into markdown

Rewrite code block start tags, changing rustdoc into equivalent in markdown:
- "```", "```no_run", "```ignore" and "```should_panic" are converted to "```rust"
- markdown heading are indentend to be one level lower, so the crate name is at the top level

Code taken almost verbatim from <https://github.com/livioribeiro/cargo-readme/blob/a7fd456433832f873cee890a9e67ece9929fc795/src/readme/process.rs>.

License: Apache-2.0
