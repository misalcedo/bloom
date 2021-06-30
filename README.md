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
* Generate a Bundle from the C dynamic library using Ruby's mkmf tooling with `rake compile`.

Then, run `rake` to execute unit tests or `rake console` to run an IRB session with the extension in scope.

Lastly, use the provided `Bloom::BloomFilter` class.

```
irb(main):001:0> Bloom::BloomFilter::new(42)
=> #<Bloom::BloomFilter:0x00007fc7a70b0970>
```

# Benchmark
The benchmarks can be run via `rake bench`.

```
codespace ➜ /workspaces/bloom (concurrent ✗) $ rake bench
                         user     system      total        real
Pure Ruby                4.125050   0.000073   4.125123 (  4.125702)
Rust via C-API           3.338941   0.000000   3.338941 (  3.338999)
Rust via FFI gem         3.666967   0.000000   3.666967 (  3.667031)
Atomic Pure Ruby         4.297110   0.000000   4.297110 (  4.297199)
Atomic Rust via C-API    3.536516   0.000000   3.536516 (  3.536612)
Atomic Rust via FFI gem  3.823249   0.003993   3.827242 (  3.827277)
```

# Resources
* [Extending Ruby with C-extensions](https://ruby-doc.com/docs/ProgrammingRuby/html/ext_ruby.html)
* [bindgen User Guide](https://rust-lang.github.io/rust-bindgen/introduction.html)
* [CLang Bundles](https://clang-build.readthedocs.io/en/latest/user_guide/bundling.html)
* [Ruby](https://github.com/ruby/ruby)
* [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
* [Ruby FFI Gem](https://github.com/ffi/ffi/wiki)
