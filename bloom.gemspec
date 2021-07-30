Gem::Specification.new do |spec|
  spec.name        = "bloom"
  spec.version     = '0.1.0'
  spec.licenses    = ["MIT"]
  spec.summary     = "A bloom filter implementation written in Rust."
  spec.description = "A bloom filter implementation written in Rust as a Ruby C-extension."
  spec.authors     = ["Miguel D. Salcedo"]
  spec.email       = "misalcedo@github.com"
  spec.homepage    = "https://github.com/misalcedo/bloom"
  spec.metadata    = { "source_code_uri" => "https://github.com/misalcedo/bloom" }

  # Configuration to build a signed gem.
=begin
  spec.cert_chain  = ['certs/misalcedo.pem']
  spec.signing_key = File.expand_path("~/.ssh/gem-private_key.pem") if $0 =~ /gem\z/
=end

  spec.files       = Dir["src/*"] + Dir["extension/*"] + %w[Cargo.toml build.rs lib/bloom_ruby.rb lib/bloom_ffi.rb]
  spec.extensions  = ["Rakefile"]
  spec.require_paths = %w[lib target/release]

  spec.add_development_dependency "bundler", ">=2.2"
  spec.add_development_dependency "rake", ">=13.0"
  spec.add_development_dependency "rspec", ">=3.10"

  spec.add_runtime_dependency "ffi", ">=1.15"
  spec.add_runtime_dependency "concurrent-ruby", ">=1.1"
  spec.add_runtime_dependency "parallel", ">=1.20"

  spec.requirements << "Rust"
  spec.requirements << "libopenssl"
end
