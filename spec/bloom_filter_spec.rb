require 'bloom_filter'

RSpec.describe Bloom::BloomFilter do
  it "#new" do
    expect { described_class.new(42) }.to_not raise_error
  end

  describe "with valid capacity" do
    subject { described_class.new(1) }

    it "#capacity" do
        expect(subject.capacity).to be >= 1
    end

    describe "#contains" do
      it "when not in the data structure" do
        expect(subject.contains("foo")).to eq false
      end

      it "when in the data structure" do
        subject.insert("foo")
        expect(subject.contains("foo")).to eq true
      end

      skip "when potentially in the data structure", "until the bloom filter is probabilistic" do
        subject.insert("bar")
        expect(subject.contains("foo")).to eq true
      end
    end
  end
end
