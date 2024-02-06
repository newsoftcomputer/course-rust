
# Command line apps in Rust
    
    - Rust is a statically compiled, fast language with great tooling and a rapidly growing ecosystem. That makes it a great fit for writing command line applications: They should be small, portable, and quick to run. Command line applications are also a great way to get started with learning Rust; or to introduce Rust to your team!

    - Writing a program with a simple command line interface (CLI) is a great exercise for a beginner who is new to the language and wants to get a feel for it. There are many aspects to this topic, though, that often only reveal themselves later on.

    - This book is structured like this: We start with a quick tutorial, after which you’ll end up with a working CLI tool. You’ll be exposed to a few of the core concepts of Rust as well as the main aspects of CLI applications. What follows are chapters that go into more detail on some of these aspects.

    - One last thing before we dive right into CLI applications: If you find an error in this book or want to help us write more content for it, you can find its source in the CLI book repository. We’d love to hear your feedback! Thank you!



## Learning Rust by Writing a Command Line App in 15 Minutes
    
    - This tutorial will guide you through writing a CLI (command line interface) application in Rust. It will take you roughly fifteen minutes to get to a point where you have a running program (around chapter 1.3). After that, we’ll continue to tweak our program until we reach a point where we can ship our little tool.

    - You’ll learn all the essentials about how to get going, and where to find more information. Feel free to skip parts you don’t need to know right now or jump in at any point.

    - Prerequisites: This tutorial does not replace a general introduction to programming, and expects you to be familiar with a few common concepts. You should be comfortable with using a command line/terminal. If you already know a few other languages, this can be a good first contact with Rust.

    - Getting help: If you at any point feel overwhelmed or confused with the features used, have a look at the extensive official documentation that comes with Rust, first and foremost the book, The Rust Programming Language. It comes with most Rust installations (rustup doc), and is available online on doc.rust-lang.org.

    - You are also very welcome to ask questions – the Rust community is known to be friendly and helpful. Have a look at the community page to see a list of places where people discuss Rust.

    - What kind of project do you want to write? How about we start with something simple: Let’s write a small grep clone. That is a tool that we can give a string and a path and it’ll print only the lines that contain the given string. Let’s call it grrs (pronounced “grass”).

    - In the end, we want to be able to run our tool like this:

        $ cat test.txt
        foo: 10
        bar: 20
        baz: 30
        $ grrs foo test.txt
        foo: 10
        $ grrs --help
        [some help text explaining the available options]

    
    - Note: This book is written for Rust 2018. The code examples can also be used on Rust 2015, but you might need to tweak them a bit; add extern crate foo; invocations, for example.

    - Make sure you run Rust 1.31.0 (or later) and that you have edition = "2018" set in the [package] section of your Cargo.toml file.
