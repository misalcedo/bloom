require "ffi"

module BloomFFI
  extend FFI::Library

  ffi_lib "bloom"

  class BloomFilterPointer < ::FFI::AutoPointer
    def self.release(ptr)
      BloomFFI.bloom_filter_free(ptr)
    end
  end

  attach_function :bloom_new, [:size_t], BloomFilterPointer
  attach_function :bloom_drop, [BloomFilterPointer], :void
  attach_function :bloom_capacity, [ BloomFilterPointer ], :size_t
  attach_function :bloom_insert, [ BloomFilterPointer, :string ], :bool
  attach_function :bloom_contains, [ BloomFilterPointer, :string ], :bool

  class BloomFilter
    def initialize(capacity)
      @ptr = BloomFFI.bloom_new(capacity)
    end

    def capacity
      BloomFFI.bloom_capacity(@ptr)
    end

    def add(value)
      BloomFFI.bloom_insert(@ptr, value)
    end

    def include?(value)
      BloomFFI.bloom_contains(@ptr, value)
    end
  end
end
