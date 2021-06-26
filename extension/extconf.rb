require "mkmf"

have_library("bloom", "BloomFilterNew")
have_header("bloom.h")
create_makefile("bloom_filter")