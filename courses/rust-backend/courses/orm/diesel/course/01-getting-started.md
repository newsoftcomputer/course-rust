
# Getting Started with Diesel

    - For this guide, we’re going to walk through some simple examples for each of the pieces of CRUD, which stands for “Create Read Update Delete”. Each step in this guide will build on the previous and is meant to be followed along.

    - Before we start, make sure you have PostgreSQL installed and running. If you are using some different database, like for example SQLite, some examples won’t just run as the implemented API might differ. In the project repository, you may find various examples for every supported database.

    - A note on Rust versions:
    Diesel requires Rust 1.78 or later. If you’re following along with this guide, make sure you’re using at least that version of Rust by running rustup update stable.