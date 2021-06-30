require 'bloom_filter'
require 'bloom_ruby'
require 'bloom_ffi'

RSpec.shared_examples "is a bloom filter" do
  it "#new" do
    expect { described_class.new(42) }.to_not raise_error
  end

  describe "with capacity of 1" do
    subject { described_class.new(1) }

    it "#capacity" do
        expect(subject.capacity).to be >= 1
    end

    describe "#include?" do
      it "when not in the data structure" do
        expect(subject).to_not include("foo")
      end

      it "when in the data structure" do
        subject.add("foo")
        expect(subject).to include("foo")
      end

      it "when potentially in the data structure" do
        subject.add("bar")
        expect(subject).to include("foo")
      end
    end
  end

  describe "with capacity of 100" do
    subject { described_class.new(100) }

    describe "#include?" do
      it "when not in the data structure" do
        expect(subject).to_not include("foo")
      end

      it "when in the data structure" do
        subject.add("foo")
        expect(subject).to include("foo")
      end

      it "when potentially in the data structure" do
        subject.add("bar")
        expect(subject).to_not include("foo")
      end
    end
  end
end

RSpec.describe Bloom::BloomFilter do
  include_examples "is a bloom filter"
end

RSpec.describe Bloom::AtomicBloomFilter do
  include_examples "is a bloom filter"
end

RSpec.describe BloomRuby::BloomFilter do
  include_examples "is a bloom filter"
end

RSpec.describe BloomRuby::AtomicBloomFilter do
  include_examples "is a bloom filter"
end

RSpec.describe BloomFFI::BloomFilter do
  include_examples "is a bloom filter"
end

RSpec.describe BloomFFI::AtomicBloomFilter do
  include_examples "is a bloom filter"
end