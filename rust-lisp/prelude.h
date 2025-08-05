#pragma once 
#include "stdint.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
struct bug_node_t;
typedef enum{
	bug_ptr,
	bug_integer,
	bug_double,
	bug_char,
	bug_bool,	
	bug_char_ptr,
	bug_string,
	bug_void_fn, 
	bug_non_void_fn,
	bug_list_ptr,
	bug_list_non_ptr,
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

typedef struct bug_context_t{
	bug_node_t * base_ptr;
	bug_node_t * stack;
	bug_node_t * stack_ptr;
	bug_node_t * stack_end;
	bug_node_t * captures;
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

