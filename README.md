# Bloom
A [bloom filter](https://en.wikipedia.org/wiki/Bloom_filter) implementation written in Rust to as a Ruby C-extension.

# Prerequisites
To run the code you must have the following tools:
* Ruby
* Rust
* CLang

# Execute
To run the example, from the root of the repository, run `rake`. This will:
* Compile the rust crate.
* Generate a Bundle from the C dynamic library using Ruby's RBConfig.

Then, run `rake console` to run an IRB session with the extension in scope.

# Resources
* [Extending Ruby with C-extensions](https://ruby-doc.com/docs/ProgrammingRuby/html/ext_ruby.html)
* [bindgen User Guide](https://rust-lang.github.io/rust-bindgen/introduction.html)
