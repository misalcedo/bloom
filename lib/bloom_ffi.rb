require "ffi"

module BloomFFI

  module Library
    extend FFI::Library
    ffi_lib "bloom"

    attach_function :BloomFilterDrop, [:pointer, :size_t], :pointer # returns fresh memory
    attach_function :bloom_filter_initialize, [ :pointer ], :void             # frees memory
    attach_function :bloom_filter_new, [ :pointer ], :void             # frees memory
    attach_function :bloom_filter_capacity, [ :pointer ], :void             # frees memory
    attach_function :bloom_filter_initialize, [ :pointer ], :void             # frees memory
    attach_function :bloom_filter_initialize, [ :pointer ], :void             # frees memory

  end

  class AutoPointer < ::FFI::AutoPointer
    # This method will be called by FFI::AutoPointer::DefaultReleaser.
    def self.release(ptr)
      Library.bloom_filter_free(ptr)
    end
  end
end
