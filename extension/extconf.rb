require "mkmf"

dir_config("bloom")
have_library("bloom", "bloom_new")
have_header("bloom.h")
create_makefile("bloom_filter")
