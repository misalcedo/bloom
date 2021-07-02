require "ffi"

module BloomFFI
  extend FFI::Library

  BloomFilterError = Class.new(StandardError)

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

  ErrorKind = enum :NotSupported, :AsyncRuntime
  Tag = enum :Ok, :Err

  class Error < FFI::Struct
    layout :kind, ErrorKind
  end

  class Ok < FFI::Struct
    layout :ok, :bool
  end

  class Err < FFI::Struct
    layout :err, Error
  end

  class Result < FFI::Union
    layout :ok, Ok,
            :err, Err
  end
  
  class BloomResult < FFI::Struct
    layout :tag, Tag,
            :result, Result

    def ok?
      self[:tag] == :Ok
    end

    def error?
      self[:tag] == :Err
    end

    def value
      self[:result][:ok][:ok] if ok?
    end

    def error_kind
      self[:result][:err][:err][:kind] if error?
    end
  end

  attach_function :bloom_new, [:size_t], BloomFilterPointer
  attach_function :bloom_drop, [BloomFilterPointer], :void
  attach_function :bloom_capacity, [ BloomFilterPointer ], :size_t
  attach_function :bloom_insert, [ BloomFilterPointer, :string ], :bool
  attach_function :bloom_contains, [ BloomFilterPointer, :string ], :bool
  attach_function :bloom_contains_all, [ BloomFilterPointer, :pointer, :size_t ], BloomResult.by_value
  attach_function :bloom_remove, [ BloomFilterPointer, :string ], BloomResult.by_value

  attach_function :atomic_bloom_new, [:size_t], AtomicBloomFilterPointer
  attach_function :atomic_bloom_drop, [AtomicBloomFilterPointer], :void
  attach_function :atomic_bloom_capacity, [ AtomicBloomFilterPointer ], :size_t
  attach_function :atomic_bloom_insert, [ AtomicBloomFilterPointer, :string ], :bool
  attach_function :atomic_bloom_contains, [ AtomicBloomFilterPointer, :string ], :bool
  attach_function :atomic_bloom_contains_all, [ AtomicBloomFilterPointer, :pointer, :size_t ], BloomResult.by_value
  attach_function :atomic_bloom_remove, [ AtomicBloomFilterPointer, :string ], BloomResult.by_value

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

    def include_all?(values)
      buffer = FFI::MemoryPointer.new(:string, values.size)
      buffer.write_array_of_pointer(values.map {|s| FFI::MemoryPointer.from_string(s)})

      result = BloomFFI.bloom_contains_all(@ptr, buffer, values.size)

      if result.ok?
        result.value
      elsif result.error? && result.error_kind == :AsyncRuntime
        raise(BloomFilterError, "Bloom filter was unable to test all values.")
      else
        raise("Invalid result type.")
      end
    end

    def delete(value)
      result = BloomFFI.bloom_remove(@ptr, value)
      
      if result.ok?
        result.value
      elsif result.error? && result.error_kind == :NotSupported
        raise(BloomFilterError, "Bloom filter does not support the #delete operation.")
      else
        raise("Invalid result type.")
      end
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

    def include_all?(values)
      buffer = FFI::MemoryPointer.new(:strptr, values.size)
      buffer.write_array_of_pointer(values.map {|s| FFI::MemoryPointer.from_string(s)})

      result = BloomFFI.atomic_bloom_contains_all(@ptr, buffer, values.size)

      if result.ok?
        result.value
      elsif result.error? && result.error_kind == :AsyncRuntime
        raise(BloomFilterError, "Bloom filter was unable to test all values.")
      else
        raise("Invalid result type.")
      end
    end

    def delete(value)
      result = BloomFFI.atomic_bloom_remove(@ptr, value)
      
      if result.ok?
        result.value
      elsif result.error? && result.error_kind == :NotSupported
        raise(BloomFilterError, "Bloom filter does not support the #delete operation.")
      else
        raise("Invalid result type.")
      end
    end
  end
end
