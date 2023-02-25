# capstone-compiler

A Rust implementation of the Lox language from [Crafting Interpreters](https://craftinginterpreters.com) by Robert Nystrom. Some differences will be added 
occasionally to give our own unique twist to it.

## How To Test

In order to test the compiler you can run the REPL by not specifying any parameters:
```bash
cargo run
```

We have included a *test.txt* that you can edit and run
```bash
cargo run test.lox
```
You can use any text file you want, just make sure you specify the path well (from the root folder) or place the file in the root folder
