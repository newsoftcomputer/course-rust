
# CREATE PROJECT WITH CARGO

    - - This section provides a quick sense for the cargo command line tool. We demonstrate its ability to generate a new package for us, its ability to compile the crate within the package, and its ability to run the resulting program.


    - To start a new package with Cargo, use cargo new:

    $ $ cargo new hello_world

    - Cargo defaults to --bin to make a binary program. To make a library, we would pass --lib, instead.


## Create project classic

    - Create project with cargo

        $ cargo new hello-cargo


## Create project library

    - 

        $ cargo new --lib my-lib