require 'set'
require 'openssl'

module BloomBench
  class BloomFilter
    def initialize(capacity)
      @digest = 'SHA512'
      @markers = Array.new(capacity, false)
    end

    def capacity
      @markers.size
    end

    def add(value)
      contained = true

      digest = OpenSSL::Digest.new(@digest).digest(value.to_s)
      digest.unpack("J*").each do |i|
        index = i % self.capacity
        contained &&= @markers[index]
        @markers[index] = true
      end

      contained
    end

    def include?(value)
      digest = OpenSSL::Digest.new(@digest).digest(value.to_s)
      digest.unpack("J*").all? { |i| @markers[index = i % self.capacity] }
    end
  end

  def self.exercise(items=1_000, &block)
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
Benchmark.bm do |x|
  x.report("Pure Ruby") do
    n.times do
      BloomBench.exercise { |capacity| BloomBench::BloomFilter.new(capacity) }
    end
  end

  x.report("Rust via C-API") do
    n.times do
      BloomBench.exercise { |capacity| Bloom::BloomFilter.new(capacity) }
    end
  end

  x.report("Rust via FFI gem") do
    n.times do
      #TODO
    end
  end
end
