#include <ruby.h>
#include <bloom.h>

static void bloom_filter_free(void *p) {
    drop_bloom_filter(p);
}

static VALUE bloom_filter_initialize(VALUE self)
{
    return self;
}

VALUE bloom_filter_new(VALUE class) {
    BloomFilter *bloom_filter = new_bloom_filter();

    VALUE tdata = Data_Wrap_Struct(class, 0, bloom_filter_free, bloom_filter);
    VALUE argv[0];

    rb_obj_call_init(tdata, 0, argv);

    return tdata;
}

void Init_bloom_filter() {
    VALUE class = rb_define_class("BloomFilter", rb_cObject);
    rb_define_singleton_method(class, "new", bloom_filter_new, 0);
    rb_define_method(class, "initialize", bloom_filter_initialize, 0);
}