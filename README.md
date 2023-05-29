# ObsidianHtml in Rust
This project is mostly meant for me to learn Rust.

It aims to provide a library for creating an ObsidianHtml module in Rust, as well as a couple concrete modules that use it.

If I manage to get a major convert step encoded in Rust, it might actually become useful for users with very large vaults 
to run parts of ObsidianHtml in Rust, but for now, exchanging the Python `parse_metadata` module with the enclosed rust `parse_metadata` will only save you `0.03886661306023598 - 0.00858918100129813 -> 30.3 microseconds` per run of a medium large vault :)

For usage in ObsidianHtml, see:
- https://obsidian-html.github.io/v4/configurations/modules/developer-documentation/running-a-binary-module.html
