require "ffi"

module BloomFFI
  extend FFI::Library

  LIBRARY_NAME = "bloom"

  ffi_lib [LIBRARY_NAME] + $LOAD_PATH.map {|p| File.join(p, FFI::map_library_name(LIBRARY_NAME)) }

  class BloomFilterPointer < ::FFI::AutoPointer
    def self.release(ptr)
      BloomFFI.bloom_drop(ptr)
    end
  end

  class AtomicBloomFilterPointer < ::FFI::AutoPointer
    def self.release(ptr)
      BloomFFI.atomic_bloom_drop(ptr)
    end
  end

  attach_function :bloom_new, [:size_t], BloomFilterPointer
  attach_function :bloom_drop, [BloomFilterPointer], :void
  attach_function :bloom_capacity, [ BloomFilterPointer ], :size_t
  attach_function :bloom_insert, [ BloomFilterPointer, :string ], :bool
  attach_function :bloom_contains, [ BloomFilterPointer, :string ], :bool

  attach_function :atomic_bloom_new, [:size_t], AtomicBloomFilterPointer
  attach_function :atomic_bloom_drop, [AtomicBloomFilterPointer], :void
  attach_function :atomic_bloom_capacity, [ AtomicBloomFilterPointer ], :size_t
  attach_function :atomic_bloom_insert, [ AtomicBloomFilterPointer, :string ], :bool
  attach_function :atomic_bloom_contains, [ AtomicBloomFilterPointer, :string ], :bool

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

  class AtomicBloomFilter
    def initialize(capacity)
      @ptr = BloomFFI.atomic_bloom_new(capacity)
    end

    def capacity
      BloomFFI.atomic_bloom_capacity(@ptr)
    end

    def add(value)
      BloomFFI.atomic_bloom_insert(@ptr, value)
    end

    def include?(value)
      BloomFFI.atomic_bloom_contains(@ptr, value)
    end
  end
end
