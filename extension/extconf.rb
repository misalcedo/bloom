require "mkmf"

have_library("libbloom", "BloomFilterNew")
create_makefile("bloom_filter")