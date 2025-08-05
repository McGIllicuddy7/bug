#include "prelude.h"
typedef struct {
}bug_heap_t;
void  mark_reachable_from(bug_node_t * node){
	
}
void gc_collect(bug_node_t * base, bug_node_t *end, bug_heap_t * heap){
	
}
void * gc_alloc(bug_context_t* context, size_t count){
	printf("allocated %zu bytes\n",count);
	return malloc(count);	
}
bug_context_t bug_create_context(){
	bug_context_t out;
	out.stack = (bug_node_t*)malloc(sizeof(bug_node_t)*16000);
	out.base_ptr = out.stack;
	out.stack_ptr = out.stack;
	out.stack_end = out.stack+16000;
	return out;
}
bug_context_t bug_reserve_stack_space(bug_context_t * context,size_t object_count);


bug_node_t bug_to_stringlong(bug_context_t *context){
	char buff[100];
	bug_node_t b =*context->stack;
	snprintf(buff, 99, "%ld", b.car.integer);
	bug_node_t out;
	out.vtype = bug_string;
	size_t l = strlen(buff);
	out.car.char_ptr = (char*)gc_alloc(context,l);
	memcpy(out.car.char_ptr, buff, l);
	out.cdr.integer = l;
	return out;
}
bug_node_t bug_to_stringdouble(bug_context_t *context){
	char buff[100];
	bug_node_t b =*context->stack;
	snprintf(buff, 99, "%f", b.car.db);
	bug_node_t out;
	out.vtype = bug_string;
	size_t l = strlen(buff);
	out.car.char_ptr = (char*)gc_alloc(context,l);
	memcpy(out.car.char_ptr, buff, l);
	out.cdr.integer = l;
	return out;
}
bug_node_t bug_printbug_string(bug_context_t * context){
	bug_node_t out = {};
	out.vtype = bug_integer;
	out.car.integer = printf("%.*s",(int)(context->stack->cdr.integer), context->stack->car.char_ptr);
	out.cdr.integer =0;
	return out;
}
bug_node_t bug_printlnbug_string(bug_context_t * context){
	bug_node_t out = {};
	out.vtype = bug_integer;
	out.car.integer = printf("%.*s\n",(int)(context->stack->cdr.integer), context->stack->car.char_ptr);
	out.cdr.integer =0;
	return out;
}
bug_node_t to_bug_string(bug_context_t * context,const char * chars){
	bug_node_t out;
	out.vtype = bug_string;
	long l = strlen(chars);
	out.car.char_ptr = (char*)gc_alloc(context,l);
	memcpy(out.car.char_ptr,chars, l);
	out.cdr.integer = l;
	return out;
}
bug_node_t * bug_make_captures(bug_context_t* context, int* values, size_t count){
	bug_node_t * out = (bug_node_t*)gc_alloc(context,sizeof(bug_node_t)*count);
	for(size_t i =0; i<count; i++){
		out[i] = context->stack[values[i]];
	}
	return out;
}
