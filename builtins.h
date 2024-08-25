#pragma once
#include <stdio.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
typedef struct {const char * start; size_t len;}String;
typedef struct {void * start; size_t len;}VoidSlice;
void gc_push_frame();
void gc_pop_frame();
void gc_register_ptr(void * ptr, void (*collect_fn)(void *));
void gc_any_ptr(void * ptr);
void gc_String(void *s);
void gc_any_ptr(void*);
void user_put_str_String(String s);
void * gc_alloc(size_t size);
void gc_long(void * ptr);
void gc_String(void * ptr);
void gc_double(void * ptr);
void gc_char(void * ptr);
void gc_bool(void * ptr);
String operator_plus_String_String(String a, String b);
String user_int_to_string_long(long a);
String make_string_from(const char * str, size_t len);
ssize_t get_allocation_count();