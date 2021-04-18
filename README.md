# Graphical

A powerful, and visual tool for graphing mathematical functions and equations."

## Usage

The tool is not done yet, as we plan to add command line parsing, but it still can be tested. Make sure you have Rust, and CMake installed, as the project is written in Rust, and we're just using bindings for a C++ library known as FLTK, which uses C++.

1. Download the code from GitHub
2. In `cli.rs`, there is a compute function, and whatever gets returned gets graphed. For example, if you were to put `return x;`, it would graph a linear function. You can change the equation to whatever you would like.
3. To run, run the command `cargo run graph` in your terminal.
4. That's it!
