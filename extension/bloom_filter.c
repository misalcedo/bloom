#include <ruby.h>
#include <bloom.h>

static void bloom_filter_free(void *p) {
    BloomFilterDrop(p);
}

static VALUE bloom_filter_initialize(VALUE self)
{
    return self;
}

VALUE bloom_filter_new(VALUE class, VALUE capacity) {
    Check_Type(capacity, T_FIXNUM);

    BloomFilter *bloom_filter = BloomFilterNew(NUM2ULONG(capacity));

    VALUE tdata = Data_Wrap_Struct(class, 0, bloom_filter_free, bloom_filter);
    VALUE argv[0];

    rb_obj_call_init(tdata, 0, argv);

    return tdata;
}

static VALUE bloom_filter_capacity(VALUE self)
{
    BloomFilter *ptr;

    Data_Get_Struct(self, BloomFilter, ptr);

    unsigned long capacity = BloomFilterCapacity(ptr);

    return ULONG2NUM(capacity);
}

void Init_bloom_filter() {
    VALUE module = rb_define_module("Bloom");
    VALUE class = rb_define_class_under(module, "BloomFilter", rb_cObject);
    rb_define_singleton_method(class, "new", bloom_filter_new, 1);
    rb_define_method(class, "initialize", bloom_filter_initialize, 0);
    rb_define_method(class, "capacity", bloom_filter_capacity, 0);
}