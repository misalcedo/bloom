require "openssl"

module BloomRuby
  BloomFilterError = Class.new(StandardError)

  def self.indices_of(value, digest="SHA512")
    digest = OpenSSL::Digest.new(digest).digest(value.to_s)
    digest.unpack("J*")
  end

  class BloomFilter
    def initialize(capacity)
      @markers = Array.new(capacity, false)
    end

    def capacity
      @markers.size
    end

    def add(value)
      contained = true

      BloomRuby::indices_of(value).each do |i|
        index = i % self.capacity
        contained &&= @markers[index]
        @markers[index] = true
      end

      contained
    end

    def include?(value)
      BloomRuby::indices_of(value).all? { |i| @markers[index = i % self.capacity] }
    end


    def include_all?(values)
      values.all? { |value| include?(value) }
    end

    def delete(value)
      raise(BloomFilterError, "Bloom filter does not support the #delete operation.")
    end
  end

  class AtomicBloomFilter
    def initialize(capacity)
      @semaphore = Mutex.new
      @markers = Array.new(capacity, false)
    end

    def capacity
      @markers.size
    end

    def add(value)
      contained = true

      @semaphore.synchronize do
        BloomRuby::indices_of(value).each do |i|
          index = i % self.capacity
          contained &&= @markers[index]
          @markers[index] = true
        end
      end

      contained
    end

    def include?(value)
      @semaphore.synchronize do
        BloomRuby::indices_of(value).all? { |i| @markers[index = i % self.capacity] }
      end
    end

    def include_all?(values)
      values.all? { |value| include?(value) }
    end

    def delete(value)
      raise(BloomFilterError, "Bloom filter does not support the #delete operation.")
    end
  end
end
