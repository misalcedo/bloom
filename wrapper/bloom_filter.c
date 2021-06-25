#include <ruby.h>
#include <bloom.h>

void Init_bloom_filter() {
    rb_define_class("BloomFilter", rb_cObject);
}