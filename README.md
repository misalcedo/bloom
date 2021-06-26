# Bloom
A [bloom filter](https://en.wikipedia.org/wiki/Bloom_filter) implementation written in Rust to as a Ruby C-extension.

# Prerequisites
To run the code you must have the following tools (only tested on macOS):
* Ruby (tested on 3.0.1 installed with [rbenv](https://github.com/rbenv/rbenv))
* Rust
* CLang

# Execute
To run the example, from the root of the repository, run `rake`. This will:
* Install Ruby dependencies with `bundler install`.
* Compile the rust crate with `cargo build`.
* Generate a Bundle from the C dynamic library using Ruby's mkmf tooling with `rake compile_c`.

Then, run `rake test` to execute unit tests or `rake console` to run an IRB session with the extension in scope.

Lastly, use the provided `Bloom::BloomFilter` class.

```
irb(main):001:0> require "bloom_filter"
=> true
irb(main):002:0> Bloom::BloomFilter::new(42)
=> #<Bloom::BloomFilter:0x00007fc7a70b0970>
```

# Resources
* [Extending Ruby with C-extensions](https://ruby-doc.com/docs/ProgrammingRuby/html/ext_ruby.html)
* [bindgen User Guide](https://rust-lang.github.io/rust-bindgen/introduction.html)
* [CLang Bundles](https://clang-build.readthedocs.io/en/latest/user_guide/bundling.html)
* [Ruby](https://github.com/ruby/ruby)
