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
1. Change directory to `examples`.
1. Run `bundle install`.
1. Run `bundle exec irb` or `bundle exec ruby`.

# Install
To install this gem locally:

1. Run `gem build bloom`.
1. Run `gem install bloom`.
1. Run `irb` or `ruby`.

# Notes
Running `gem build bloom` produces a nested GZipped tar file (see `gem help build` for instructions on how to unpack a gem). The inside the `.gem` file is a `data.tar.gz` file that contains the items in the gemspec's `files` list. This gives us 2 options for packaging gems for consumption:
1. Build Rust library in a CI pipeline to produce the library. Then, add the C dynamic library to the gem.
1. Add the rust source code to the gem output. Then, build Rust library on the target machine. Thus, requiring a local version of the Rust toolchain (i.e. rustup, rustc, and cargo). to produce the library.

Building on the target machine is beneficial to ensure our gem can be downloaded from a central store and will work anywhere. However, for internal-only gems building in a CI pipeline would greatly reduce installation time.

Also, the result type returned in the remove methods has no Drop trait logic. Therefore, we can safely return it by value. Otherwise, we could treat the result as a managed struct and call a drop method on it. However, this would only be supported automatically in the FFI case. The C library would need to introduce calls to free the result wherever necessary.

# Security
We can sign gem output and install with high security to secure our supply chain. See [RubyGems - Security](https://guides.rubygems.org/security/) for more details.

# Benchmark
The benchmarks can be run via `rake bench`. The following run was captured using a GitHub workspace:

```
$ rake bench
                                               user     system      total        real
Pure Ruby                                  0.390053   0.000000   0.390053 (  0.390063)
Rust via C-API                             0.067710   0.000000   0.067710 (  0.067710)
Rust via FFI gem                           0.093329   0.000000   0.093329 (  0.093329)
Atomic Pure Ruby                           0.400783   0.000000   0.400783 (  0.400786)
Atomic Rust via C-API                      0.071238   0.000000   0.071238 (  0.071241)
Atomic Rust via FFI gem                    0.100829   0.000000   0.100829 (  0.100828)
Atomic Pure Ruby with Concurrency          4.362044   1.713511   8.535082 (  4.651307)
Atomic Rust via C-API with Concurrency     1.912113   0.000000   1.912113 (  1.840493)
Atomic Rust via FFI gem with Concurrency   2.145017   0.000000   2.145017 (  2.076937)
```

# Resources
* [Extending Ruby with C-extensions](https://ruby-doc.com/docs/ProgrammingRuby/html/ext_ruby.html)
* [bindgen User Guide](https://rust-lang.github.io/rust-bindgen/introduction.html)
* [CLang Bundles](https://clang-build.readthedocs.io/en/latest/user_guide/bundling.html)
* [Ruby](https://github.com/ruby/ruby)
* [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
* [Ruby FFI Gem](https://github.com/ffi/ffi/wiki)
