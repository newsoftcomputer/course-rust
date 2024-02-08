
# CARGO GENERATE


    URL: https://cargo-generate.github.io/cargo-generate/index.html


    - cargo-generate is a developer tool to help you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.

    - cargo-generate uses Shopify's Liquid template language, Rhai for hook scripts and regex for placeholders.

    - Due to the use of Shopify's Liquid, cargo-generate special cases files with the file-ending .liquid, by simply removing the file-ending when processing the files. If you, as a template author, truly want the .liquid file-ending, you need to repeat it twice!

    - An Example: the file main.rs.liquid will be renamed after templating to main.rs

    - Here's an example of using cargo-generate with this template: