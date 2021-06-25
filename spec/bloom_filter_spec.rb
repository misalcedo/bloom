require 'bloom_filter'

RSpec.describe BloomFilter do
  it "#new" do
    expect { described_class.new }.to_not raise_error
  end
end