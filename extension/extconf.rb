require "mkmf"

dir_config("bloom")
have_library("bloom", "BloomFilterNew")
have_header("bloom.h")
create_makefile("bloom_filter")