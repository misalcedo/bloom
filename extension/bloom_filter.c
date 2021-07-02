#include <ruby.h>
#include <ruby/encoding.h>
#include <bloom.h>

static VALUE not_supported;

static void bloom_filter_free(void *p) {
    bloom_drop(p);
}

static VALUE bloom_filter_initialize(VALUE self)
{
    return self;
}

VALUE bloom_filter_new(VALUE class, VALUE capacity) {
    Check_Type(capacity, T_FIXNUM);

    BloomFilter *bloom_filter = bloom_new(NUM2SIZET(capacity));

    VALUE tdata = Data_Wrap_Struct(class, 0, bloom_filter_free, bloom_filter);
    VALUE argv[0];

    rb_obj_call_init(tdata, 0, argv);

    return tdata;
}

static VALUE bloom_filter_capacity(VALUE self)
{
    BloomFilter *ptr;

    Data_Get_Struct(self, BloomFilter, ptr);

    unsigned long capacity = bloom_capacity(ptr);

    return SIZET2NUM(capacity);
}

static VALUE bloom_filter_insert(VALUE self, VALUE value)
{
    Check_Type(value, T_STRING);

    BloomFilter *ptr;

    Data_Get_Struct(self, BloomFilter, ptr);

    bool already_contains = bloom_insert(ptr, RSTRING_PTR(value));

    return already_contains ? Qtrue : Qfalse;
}

static VALUE bloom_filter_contains(VALUE self, VALUE value)
{
    Check_Type(value, T_STRING);

    BloomFilter *ptr;

    Data_Get_Struct(self, BloomFilter, ptr);

    bool contains = bloom_contains(ptr, RSTRING_PTR(value));

    return contains ? Qtrue : Qfalse;
}

static VALUE bloom_filter_remove(VALUE self, VALUE value)
{
    Check_Type(value, T_STRING);

    BloomFilter *ptr;

    Data_Get_Struct(self, BloomFilter, ptr);

    BloomResult_bool result = bloom_remove(ptr, RSTRING_PTR(value));

    if (result.tag == Ok_bool)
    {
        return result.ok ? Qtrue : Qfalse;
    }
    else if (result.tag == Err_bool && result.err.kind == NotSupported)
    {
        rb_raise(not_supported, "Bloom filter does not support the #delete operation.");
    }

    return Qnil;
}

static void atomic_bloom_filter_free(void *p) {
    atomic_bloom_drop(p);
}

static VALUE atomic_bloom_filter_initialize(VALUE self)
{
    return self;
}

VALUE atomic_bloom_filter_new(VALUE class, VALUE capacity) {
    Check_Type(capacity, T_FIXNUM);

    AtomicBloomFilter *bloom_filter = atomic_bloom_new(NUM2SIZET(capacity));

    VALUE tdata = Data_Wrap_Struct(class, 0, atomic_bloom_filter_free, bloom_filter);
    VALUE argv[0];

    rb_obj_call_init(tdata, 0, argv);

    return tdata;
}

static VALUE atomic_bloom_filter_capacity(VALUE self)
{
    AtomicBloomFilter *ptr;

    Data_Get_Struct(self, AtomicBloomFilter, ptr);

    unsigned long capacity = atomic_bloom_capacity(ptr);

    return SIZET2NUM(capacity);
}

static VALUE atomic_bloom_filter_insert(VALUE self, VALUE value)
{
    Check_Type(value, T_STRING);

    AtomicBloomFilter *ptr;

    Data_Get_Struct(self, AtomicBloomFilter, ptr);

    bool already_contains = atomic_bloom_insert(ptr, RSTRING_PTR(value));

    return already_contains ? Qtrue : Qfalse;
}

static VALUE atomic_bloom_filter_contains(VALUE self, VALUE value)
{
    Check_Type(value, T_STRING);

    AtomicBloomFilter *ptr;

    Data_Get_Struct(self, AtomicBloomFilter, ptr);

    bool contains = atomic_bloom_contains(ptr, RSTRING_PTR(value));

    return contains ? Qtrue : Qfalse;
}

static VALUE atomic_bloom_filter_remove(VALUE self, VALUE value)
{
    Check_Type(value, T_STRING);

    AtomicBloomFilter *ptr;

    Data_Get_Struct(self, AtomicBloomFilter, ptr);

    BloomResult_bool result = atomic_bloom_remove(ptr, RSTRING_PTR(value));
    
    if (result.tag == Ok_bool)
    {
        return result.ok ? Qtrue : Qfalse;
    }
    else if (result.tag == Err_bool && result.err.kind == NotSupported)
    {
        rb_raise(not_supported, "Bloom filter does not support the #delete operation.");
    }

    return Qnil;
}

void Init_bloom_filter() {
    VALUE module = rb_define_module("Bloom");

    not_supported = rb_define_class_under(module, "BloomFilterError", rb_eStandardError);

    VALUE class = rb_define_class_under(module, "BloomFilter", rb_cObject);
    rb_define_singleton_method(class, "new", bloom_filter_new, 1);
    rb_define_method(class, "initialize", bloom_filter_initialize, 0);
    rb_define_method(class, "capacity", bloom_filter_capacity, 0);
    rb_define_method(class, "add", bloom_filter_insert, 1);
    rb_define_method(class, "include?", bloom_filter_contains, 1);
    rb_define_method(class, "delete", bloom_filter_remove, 1);

    VALUE atomic_class = rb_define_class_under(module, "AtomicBloomFilter", rb_cObject);
    rb_define_singleton_method(atomic_class, "new", atomic_bloom_filter_new, 1);
    rb_define_method(atomic_class, "initialize", atomic_bloom_filter_initialize, 0);
    rb_define_method(atomic_class, "capacity", atomic_bloom_filter_capacity, 0);
    rb_define_method(atomic_class, "add", atomic_bloom_filter_insert, 1);
    rb_define_method(atomic_class, "include?", atomic_bloom_filter_contains, 1);
    rb_define_method(atomic_class, "delete", atomic_bloom_filter_remove, 1);
}
