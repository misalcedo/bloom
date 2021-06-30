require "bloom_ruby"
require "bloom_filter"
require "bloom_ffi"

module BloomBench
  def self.exercise(items, &block)
    bloom_filter = block.call(42)

    raise "Invalid capacity." unless bloom_filter.capacity >= 42

    1.upto(items) do |n|
      bloom_filter.add("#{n}") if n.even?
    end

    1.upto(items) do |n|
      raise "Invalid membership check." if n.even? && !bloom_filter.include?("#{n}")
    end
  end
end

n = 1_000
i = 1_000
Benchmark.bm do |x|
  x.report("Pure Ruby") do
    n.times do
      BloomBench.exercise(i) { |capacity| BloomRuby::BloomFilter.new(capacity) }
    end
  end

  x.report("Rust via C-API") do
    n.times do
      BloomBench.exercise(i) { |capacity| Bloom::BloomFilter.new(capacity) }
    end
  end

  x.report("Rust via FFI gem") do
    n.times do
      BloomBench.exercise(i) { |capacity| BloomFFI::BloomFilter.new(capacity) }
    end
  end

  x.report("Atomic Pure Ruby") do
    n.times do
      BloomBench.exercise(i) { |capacity| BloomRuby::AtomicBloomFilter.new(capacity) }
    end
  end

  x.report("Atomic Rust via C-API") do
    n.times do
      BloomBench.exercise(i) { |capacity| Bloom::AtomicBloomFilter.new(capacity) }
    end
  end

  x.report("Atomic Rust via FFI gem") do
    n.times do
      BloomBench.exercise(i) { |capacity| BloomFFI::AtomicBloomFilter.new(capacity) }
    end
  end
end
