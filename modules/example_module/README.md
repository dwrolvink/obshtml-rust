# Introduction
This is an example module using the obshtml_module_lib crate to create a custom ObsidianHtml module with Rust.

# Compiling
Before you can run this example, you will need to compile the rust code.

To compile the code, first install rust along with cargo:
- https://www.rust-lang.org/tools/install

Then, run:
``` bash
cargo build --release
```

This will compile `./target/release/obshtml_module_example` (or `./target/release/obshtml_module_example.exe`, if you are on Windows).

> Note: this crate is not tested on Windows, nor will it be, and it (will) make extensive use of Posix paths. It will probably not work as is on Windows.

## Testing
To test your module, you can run:
``` bash
./target/release/obshtml_module_example run path/to/module_data_folder
```

> Make sure that the module data folder is already prepopulated by a previous run of ObsidianHtml.

# Use with ObsidianHtml
See:
- https://obsidian-html.github.io/v4/configurations/modules/developer-documentation/running-a-binary-module.html

