require 'bloom_filter'

RSpec.describe Bloom::BloomFilter do
  it "#new" do
    expect { described_class.new(42) }.to_not raise_error
  end

  describe "with valid capacity" do
    subject { described_class.new(42) }

    it "#capacity" do
        expect { subject.capacity }.to eq 42
    end
  end
end