
# OWNERSHIP

    - Doc: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

    - Stcak & 

## WHAT IS OWNERSHIP

    - Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

    Because ownership is a new concept for many programmers, it does take some time to get used to. The good news is that the more experienced you become with Rust and the rules of the ownership system, the easier you’ll find it to naturally develop code that is safe and efficient. Keep at it!

    When you understand ownership, you’ll have a solid foundation for understanding the features that make Rust unique. In this chapter, you’ll learn ownership by working through some examples that focus on a very common data structure: strings.


## OWNERSHIP RULES 

    - First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

    1 Rule. Each value in Rust has an owner.

    2 Rule. There can only be one owner at a time.

    3 Rule. When the owner goes out of scope, the value will be dropped.