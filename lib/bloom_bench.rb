require "bloom_ruby"
require "bloom_filter"
require "bloom_ffi"
require "concurrent-ruby"

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

  def self.concurrent_exercise(items, threads, &block)
    bloom_filter = block.call(42)

    raise "Invalid capacity." unless bloom_filter.capacity >= 42

    pool = Concurrent::FixedThreadPool.new(threads)

    additions = 1.upto(items).map do |n|
      pool.post do
        bloom_filter.add("#{n}") if n.even?
      end
    end

    nil until pool.completed_task_count == items

    1.upto(items) do |n|
      pool.post do
        raise "Invalid membership check." if n.even? && !bloom_filter.include?("#{n}")
      end
    end

    nil until pool.completed_task_count == (2 * items)
    pool.shutdown
    pool.wait_for_termination
  end
end

n = 10
i = 10_000
Benchmark.bmbm do |x|
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

  x.report("Atomic Pure Ruby with Concurrency") do
    n.times do
      BloomBench.concurrent_exercise(i, n) { |capacity| BloomRuby::AtomicBloomFilter.new(capacity) }
    end
  end

  x.report("Atomic Rust via C-API with Concurrency") do
    n.times do
      BloomBench.concurrent_exercise(i, n) { |capacity| Bloom::AtomicBloomFilter.new(capacity) }
    end
  end

  x.report("Atomic Rust via FFI gem with Concurrency") do
    n.times do
      BloomBench.concurrent_exercise(i, n) { |capacity| BloomFFI::AtomicBloomFilter.new(capacity) }
    end
  end
end
