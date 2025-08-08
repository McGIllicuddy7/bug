#pragma once 
#include "stdint.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
struct bug_node_t;
//generally cdr stores any pointers
typedef enum:size_t{
	bug_undefined_type,
	bug_ptr,
	bug_integer,
	bug_double,
	bug_char,
	bug_bool,
	bug_string,
	bug_void_fn, 
	bug_non_void_fn,
	bug_list_ptr,
	bug_list_integer,
	bug_list_double,
	bug_list_char,
	bug_list_bool,
} bug_type_t;
struct bug_context_t;
typedef union{
	void* ptr;	
	char * char_ptr;
	struct bug_node_t *node;
	double db;
	long integer;
	bool boolean;
	char character;
	struct bug_node_t (*non_void_fn)(struct bug_context_t * context);
	void (*void_fn)(struct bug_context_t * context);
} bug_value_t;
typedef struct bug_node_t{
	bug_type_t vtype;
	bug_value_t car;
	bug_value_t cdr;
}bug_node_t;
typedef struct bug_allocation_t {	
	bool reachable;
	bool moved_reachable;
	bool is_objects;
	size_t object_count;
	struct bug_allocation_t * next;
} bug_allocation_t;
typedef struct {
	bug_allocation_t * allocations;
	bug_allocation_t * tmp_allocations;
	char * temp_heap;
	char * next_tmp_alloc;
	char * temp_heap_end;
}bug_heap_t;

typedef struct bug_context_t{
	bug_node_t * base_ptr;
	bug_node_t * stack;
	bug_node_t * stack_ptr;
	bug_node_t * stack_end;
	bug_node_t * captures;
	bug_heap_t * heap;
	bool gc_lock;
}bug_context_t;
bug_context_t bug_create_context();
bug_context_t bug_reserve_stack_space(bug_context_t * context,size_t object_count);
void * gc_alloc(bug_context_t* context, size_t count);
bug_node_t bug_to_stringlong(bug_context_t *context);
bug_node_t bug_to_stringdouble(bug_context_t *context);
bug_node_t bug_printbug_string(bug_context_t * context);
bug_node_t bug_printlnbug_string(bug_context_t * context);
bug_node_t to_bug_string(bug_context_t * context,const char * chars);
bug_node_t * bug_make_captures(bug_context_t* context, int* values, size_t count);
bug_node_t bug_empty_list(bug_context_t * context);
bug_node_t bug_list_cat(bug_context_t * context, bug_node_t base, bug_node_t end);
bug_node_t bug_cdr(bug_node_t node);
bug_node_t bug_box_value(bug_context_t * context, bug_node_t base);
void gc_collect(bug_node_t * base, bug_node_t * end, bug_heap_t * heap);
bug_node_t bug_is_a(bug_node_t b, bug_type_t t);
void free_heap(bug_context_t* context);
void debug_node(bug_node_t* node);
void runtime_checkups(bug_context_t * context);
