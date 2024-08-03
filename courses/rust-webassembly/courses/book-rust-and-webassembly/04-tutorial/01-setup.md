
# Setup

    - This section describes how to set up the toolchain for compiling Rust programs to WebAssembly and integrate them into JavaScript.


## The Rust Toolchain

    - You will need the standard Rust toolchain, including rustup, rustc, and cargo.

    - Follow these instructions to install the Rust toolchain.

    - The Rust and WebAssembly experience is riding the Rust release trains to stable! That means we don't require any experimental feature flags. However, we do require Rust 1.30 or newer.



## Cargo Generate

    - cargo-generate helps you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.

    - Install cargo-generate with this command:


        $ cargo install cargo-generate


## NPM

    - npm is a package manager for JavaScript. We will use it to install and run a JavaScript bundler and development server. At the end of the tutorial, we will publish our compiled .wasm to the npm registry.

    - Follow these instructions to install npm.

    - If you already have npm installed, make sure it is up to date with this command:


        $ npm install npm@latest -g