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

# Example
To use this library as a gem:

1. Run `bundle install`.
1. Run `gem build`.
1. Change directory to `example`.
1. Run `bundle install`.
1. Run `bundle exec irb` or `bundle exec ruby`.

# Benchmark
The benchmarks can be run via `rake bench`.

```
codespace ➜ /workspaces/bloom (concurrent ✗) $ rake bench
                                              user     system      total        real
Pure Ruby                                 0.397027   0.000005   0.397032 (  0.397031)
Rust via C-API                            0.324624   0.008002   0.332626 (  0.332649)
Rust via FFI gem                          0.360630   0.000000   0.360630 (  0.360632)
Atomic Pure Ruby                          0.402783   0.000000   0.402783 (  0.402793)
Atomic Rust via C-API                     0.351951   0.000000   0.351951 (  0.351962)
Atomic Rust via FFI gem                   0.378851   0.000000   0.378851 (  0.378852)
Atomic Pure Ruby with Concurreny          4.257117   0.056108   4.313225 (  4.300757)
Atomic Rust via C-API with Concurreny     3.846755   0.072121   3.918876 (  3.906959)
Atomic Rust via FFI gem with Concurreny   5.116558   0.088260   5.204818 (  5.190831)
```

# Resources
* [Extending Ruby with C-extensions](https://ruby-doc.com/docs/ProgrammingRuby/html/ext_ruby.html)
* [bindgen User Guide](https://rust-lang.github.io/rust-bindgen/introduction.html)
* [CLang Bundles](https://clang-build.readthedocs.io/en/latest/user_guide/bundling.html)
* [Ruby](https://github.com/ruby/ruby)
* [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
* [Ruby FFI Gem](https://github.com/ffi/ffi/wiki)
