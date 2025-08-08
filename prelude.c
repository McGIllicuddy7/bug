#include "prelude.h"
#include "assert.h"
//every allocation either contains structured data in the form of bug_objects or just bytes (string)
bug_context_t bug_create_context(){
	bug_context_t out;
	out.heap = (bug_heap_t*)malloc(sizeof(bug_heap_t));
	out.heap->allocations =0;
	out.heap->temp_heap = (char*)malloc(16000);
	out.heap->next_tmp_alloc= out.heap->temp_heap;
	out.heap->temp_heap_end = out.heap->temp_heap+16000;
	out.heap->tmp_allocations =0;
	out.stack = (bug_node_t*)malloc(16000*sizeof(bug_node_t));
	out.stack_ptr = out.stack;
	out.stack_end = out.stack+16000;
	return out;
}
void mark_reachable_from(bug_node_t * node,bool is_root, size_t * reachable_count){
	tail_call:	
	if(!node){
		return;
	}
	if(!is_root){
		bug_allocation_t * alc = (bug_allocation_t*)(((size_t)node)-sizeof(bug_allocation_t));
		if(alc->reachable){
			return;
		}
		alc->reachable = true;
		(*reachable_count)++;
		if(!alc->is_objects){
			return;
		}
	}
	switch(node->vtype){
		case bug_undefined_type:{
			return;
		}
		case bug_ptr:{	
			node = node->cdr.node;
			is_root = false;
			goto tail_call;
			break;
		}
		case bug_void_fn:{
			assert(false);
			mark_reachable_from(node->cdr.node, false,reachable_count);
			is_root = false;
			node = node->cdr.node;
			goto tail_call;
			break;
		}
		case bug_non_void_fn:{
			assert(false);
			mark_reachable_from(node->cdr.node, false,reachable_count);
			is_root = false;
			node = node->cdr.node;
			goto tail_call;
			break;
		}
		case bug_list_ptr:{	
			mark_reachable_from(node->car.node, false,reachable_count);
			is_root = false;
			node = node->cdr.node;
			goto tail_call;
			break;
		}
		case bug_list_bool:{	
			node = node->cdr.node;
			is_root = false;
			goto tail_call;
			break;
		}
		case bug_list_char:{	
			node = node->cdr.node;
			is_root = false;
			goto tail_call;
			break;
		}
		case bug_list_integer:{
			node = node->cdr.node;
			is_root = false;
			goto tail_call;
			break;
		}
		case bug_list_double:{	
			node = node->cdr.node;
			is_root = false;
			goto tail_call;
			break;
		}
		case bug_string:{	
			void * ptr = node->cdr.node;
			bug_allocation_t* p= (bug_allocation_t *)ptr;
			p--;
			p->reachable = true;
			(*reachable_count)++;
			break;
		}
		case bug_integer:{	
			break;
		}
		case bug_double:{	
			break;
		}
		case bug_char:{	
			break;
		}
		case bug_bool:{	
			break;
		}
		default:
			printf("is a %zu\n",  node->vtype);
			assert(false);
			return;
	}
}
void gc_object_move_pointer(bug_node_t * node, void * new_ptr, void * old_ptr, bool is_root){
tail_call:	
	if(!node){
		return;
	}
	if(!is_root){
		bug_allocation_t * alc = (bug_allocation_t*)(((size_t)node)-sizeof(bug_allocation_t));
		if(alc->moved_reachable){
			return;
		}
		alc->moved_reachable = true;	
		if(!alc->is_objects){
			return;
		}
	}
	switch(node->vtype){
		case bug_undefined_type:{
			return;
		}
		case bug_ptr:{	
			if(node->cdr.node == old_ptr){
				node->cdr.node = new_ptr;
			}
			node = node->cdr.node;
			is_root = false;
			goto tail_call;
			break;
		}
		case bug_void_fn:{
			assert(false);
			gc_object_move_pointer(node->cdr.node,new_ptr, old_ptr, false);
			is_root = false;
			node = node->cdr.node;
			goto tail_call;
			break;
		}
		case bug_non_void_fn:{
			assert(false);
			gc_object_move_pointer(node->cdr.node,new_ptr, old_ptr, false);	
			is_root = false;
			node = node->cdr.node;
			goto tail_call;
			break;
		}
		case bug_list_ptr:{	
			if(node->car.node == old_ptr){
				node->car.node = new_ptr;
			}
			gc_object_move_pointer(node->car.node,new_ptr, old_ptr, false);
			is_root = false;
			if(node->cdr.node == old_ptr){
				node->cdr.node = new_ptr;
			}	
			node = node->cdr.node;
			goto tail_call;
			break;
		}
		case bug_list_bool:{	
			if(node->cdr.node == old_ptr){
				node->cdr.node = new_ptr;
			}
			node = node->cdr.node;
			is_root = false;
			goto tail_call;
			break;
		}
		case bug_list_char:{	
			if(node->cdr.node == old_ptr){
				node->cdr.node = new_ptr;
			}
			node = node->cdr.node;
			is_root = false;
			goto tail_call;
			break;
		}
		case bug_list_integer:{
			if(node->cdr.node == old_ptr){
				node->cdr.node = new_ptr;
			}
			node = node->cdr.node;
			is_root = false;
			goto tail_call;
			break;
		}
		case bug_list_double:{	
			if(node->cdr.node == old_ptr){
				node->cdr.node = new_ptr;
			}
			node = node->cdr.node;
			is_root = false;
			goto tail_call;
			break;
		}
		case bug_string:{	
			if(node->cdr.node == old_ptr){
				node->cdr.node = new_ptr;
			}
			break;
		}
		case bug_integer:{	
			break;
		}
		case bug_double:{	
			break;
		}
		case bug_char:{	
			break;
		}
		case bug_bool:{	
			break;
		}
		default:
			printf("is a %zu\n",  node->vtype);
			assert(false);
			return;
	}
}
void gc_move_pointer(void * new_ptr, void * old_ptr, bug_node_t * start, bug_node_t * end,bug_heap_t * heap){
	bug_allocation_t * cur = heap->allocations;
	while(cur){
		cur->moved_reachable = false;
		cur = cur->next;
	}
	cur = heap->tmp_allocations;
	while(cur){
		cur->moved_reachable = false;
		cur = cur->next;
	}
	printf("setting to %p\n", new_ptr);
	for(bug_node_t * i =start; i != end; i++){
		gc_object_move_pointer(i, new_ptr, old_ptr,true);
	}
}
void gc_collect(bug_node_t * base, bug_node_t * end, bug_heap_t * heap){	
	bug_allocation_t * cur = heap->allocations;
	size_t allocation_count =0;
	while(cur){
		cur->reachable =0;
		cur = cur->next;
		allocation_count++;
	}
	cur = heap->tmp_allocations;
	while(cur){
		cur->reachable =0;
		cur = cur->next;
		allocation_count++;
	}
	size_t reachable_count =0;
	for(bug_node_t *i = base; i != end; i++){	
		mark_reachable_from(i,true, &reachable_count);
	}
	printf("%zu allocations, %zu reachable\n", allocation_count, reachable_count);
	cur = heap->allocations;
	bug_allocation_t * prev =0;
	while(cur){
		if(!cur->reachable){
			bug_allocation_t* nxt= cur->next;
			if(prev){
				prev->next=  nxt;
			} else{
				heap->allocations = nxt;
			}
			free(cur);
			cur = nxt;
		}else{
			prev = cur;
			cur = cur->next;
		}
	}
	if(!prev){
		heap->allocations = 0;
	}
	cur = heap->tmp_allocations;	
	while(cur){
		if(cur->reachable){
			size_t count = cur->is_objects? cur->object_count*sizeof(bug_node_t) : cur->object_count;
			printf("count: %zu ", count);
			count += sizeof(bug_allocation_t);
			if(count %(2*sizeof(size_t)) !=0){
				count += (2*sizeof(size_t))-count%(2*sizeof(size_t));
			}
			printf("desired size:%zu sizeof(bug_allocation-t):%zu sizeof(bug_node_t):%zu ", count, sizeof(bug_allocation_t), sizeof(bug_node_t));
			bug_allocation_t * new_ptr = (bug_allocation_t*)malloc(count);
			memcpy(new_ptr, cur, count);
			new_ptr->next = heap->allocations;
			heap->allocations = new_ptr;
			printf("%p, %p\n", new_ptr, new_ptr+1);
			gc_move_pointer(new_ptr+1,cur+1,base, end,heap);
		}
		cur = cur->next;
	}
	heap->tmp_allocations =0;
	heap->next_tmp_alloc = heap->temp_heap;	
}
void * gc_alloc(bug_context_t * context, size_t byte_count){
	size_t count = byte_count+sizeof(bug_allocation_t);
	if(count %(2*sizeof(size_t))!=0){
		count += (2*sizeof(size_t))-count%(2*sizeof(size_t));
	}
	if(context->heap->next_tmp_alloc+count>context->heap->temp_heap_end){
		bug_allocation_t * out = (bug_allocation_t*) malloc(count);
		if(!out){
			return 0;
		}
		out->is_objects = false;
		out->reachable = false;
		out->next = context->heap->allocations;
		out->object_count = byte_count;
		context->heap->allocations = out;
		return out+1;
	}
	bug_allocation_t * out = (bug_allocation_t*)context->heap->next_tmp_alloc;
	context->heap->next_tmp_alloc+= count;
	out->is_objects = false;
	out->reachable = false;
	out->next = context->heap->tmp_allocations;
	out->object_count = byte_count;
	context->heap->tmp_allocations = out;
	return out+1;
}
bug_node_t * gc_new(bug_context_t * context, uint32_t object_count){		
	size_t count = object_count*sizeof(bug_node_t)+sizeof(bug_allocation_t);
	if(count %(2*sizeof(size_t))!=0){
		count += (2*sizeof(size_t))-count%(2*sizeof(size_t));
	}
	if(context->heap->next_tmp_alloc+count>context->heap->temp_heap_end){
		bug_allocation_t * out = (bug_allocation_t*) malloc(count);
		if(!out){
			return 0;
		}
		out->is_objects = true;
		out->reachable = false;
		out->next = context->heap->allocations;
		out->object_count = object_count;
		context->heap->allocations = out;
		return (bug_node_t*)(out+1);
	}
	bug_allocation_t * out = (bug_allocation_t*)context->heap->next_tmp_alloc;
	context->heap->next_tmp_alloc+= count;
	out->is_objects = true;
	out->reachable = false;
	out->next = context->heap->tmp_allocations;
	out->object_count =object_count;
	context->heap->tmp_allocations = out;
	return (bug_node_t*)(out+1);
}
bug_node_t bug_to_stringlong(bug_context_t *context){
	char buff[100];
	bug_node_t b =*context->stack;
	snprintf(buff, 99, "%ld", b.car.integer);
	bug_node_t out;
	out.vtype = bug_string;
	size_t l = strlen(buff);
	out.cdr.char_ptr = (char*)gc_alloc(context,l+1);
	memcpy(out.cdr.char_ptr, buff, l+1);
	out.car.integer = l+1;
	return out;
}
bug_node_t bug_to_stringdouble(bug_context_t *context){
	char buff[100];
	bug_node_t b =*context->stack;
	snprintf(buff, 99, "%f", b.car.db);
	bug_node_t out;
	out.vtype = bug_string;
	size_t l = strlen(buff);
	out.cdr.char_ptr = (char*)gc_alloc(context,l+1);
	memcpy(out.cdr.char_ptr, buff, l+1);	
	out.car.integer = l+1;
	return out;
}
bug_node_t bug_printbug_string(bug_context_t * context){
	bug_node_t out = {};
	out.vtype = bug_integer;
	out.car.integer = printf("%.*s",(int)(context->stack->car.integer), context->stack->cdr.char_ptr);
	out.cdr.integer =0;
	return out;
}
bug_node_t bug_printlnbug_string(bug_context_t * context){
	bug_node_t out = {};
	out.vtype = bug_integer;
	out.car.integer = printf("%.*s\n",(int)(context->stack->car.integer), context->stack->cdr.char_ptr);
	out.cdr.integer =0;
	return out;
}
bug_node_t to_bug_string(bug_context_t * context,const char * chars){
	bug_node_t out;
	out.vtype = bug_string;
	long l = strlen(chars);
	out.cdr.char_ptr = (char*)gc_alloc(context,l+1);
	memcpy(out.cdr.char_ptr,chars, l+1);
	out.car.integer = l;
	return out;
}
bug_node_t * bug_make_captures(bug_context_t* context, int* values, size_t count){
	bug_node_t * out = gc_new(context,count+1);
	out[0].vtype = bug_integer;
	out[0].car.integer = count;
	out[0].cdr.integer =0;
	for(size_t i =1; i<count+1; i++){
		out[i] = context->stack[values[i-1]];
	}
	return out+1;
}
void free_heap(bug_context_t * context){
	free(context->stack);	
	free(context->heap->temp_heap);
	free(context->heap);
	
	context->stack =0;
}
static void debug_print_node(bug_node_t * node,int indentation){
	const char * node_type_table[] = {"ptr", "int", "double", "char", "bool", "string", "void_fn", "non_void_fn", "list_ptr", "list_non_ptr"};
	for(int i =0; i<indentation; i++){
		printf("   ");
	}
	printf("node{type: %s ",node_type_table[node->vtype]);
	if(node->vtype == bug_ptr){
		for(size_t i =0; i<node->car.integer; i++){
			debug_print_node(&node->cdr.node[i], indentation+1);
		}	
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}else if(node->vtype == bug_integer){
		printf("%ld" ,node->car.integer);
	}
	else if(node->vtype == bug_double){
		printf("%f" ,node->car.db);
	}
	else if(node->vtype == bug_char){
		printf("%c" ,node->car.character);
	}
	else if(node->vtype == bug_bool){
		printf("%b" ,node->car.boolean);
	}
	else if(node->vtype == bug_string){
		printf("\"%.*s\"" ,(int)(node->car.integer), node->cdr.char_ptr);
	}
	else if(node->vtype == bug_list_ptr ){ 
		debug_print_node(node->car.node, indentation+1);
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	else if(node->vtype == bug_list_ptr ){ 
		debug_print_node(node->car.node, indentation+1);
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	else if(node->vtype == bug_list_integer){ 
		printf("%ld\n", node->car.integer);	
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	else if(node->vtype == bug_list_double){ 
		printf("%f\n", node->car.db);	
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	else if(node->vtype == bug_list_char){ 
		printf("%c\n", node->car.character);	
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	else if(node->vtype == bug_list_bool){ 
		printf("%b\n", node->car.boolean);	
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}else{
		assert(false);
	}
	printf("}\n");


}
void debug_node(bug_node_t * node){
	debug_print_node(node,0);
}
bug_node_t bug_empty_list(bug_context_t * context){
	bug_node_t out;
	out.vtype = bug_ptr;;
	out.car.integer =1;
	out.cdr.ptr= 0;
	return out;
}
bug_node_t bug_list_cat(bug_context_t * context, bug_node_t base, bug_node_t end){	
	bug_node_t * out = (bug_node_t*)gc_new(context,1);
	bug_node_t * box =(bug_node_t*)gc_new(context, 1);
	*box = end;
	out->vtype = bug_list_ptr;
	out->cdr.node =0;
	out->car.node = box;
	if(base.cdr.node){
		bug_node_t * node = base.cdr.node;
		while(true){
			if(!node->cdr.node){
				node->cdr.node = out;
				break;
			}else{
				node = node->cdr.node;
			}
		}
	}else{
		base.cdr.node = out;
	}
	return base;	
}
bug_node_t bug_cdr(bug_node_t node){
	bug_node_t out;
	out.vtype = bug_ptr;
	out.car.integer= 1;
	out.cdr.node = node.cdr.node->cdr.node;
	return out;
}
bug_node_t bug_box_value(bug_context_t * context, bug_node_t node){
	bug_node_t * p = (bug_node_t*)gc_new(context, 1);
	*p = node;
	bug_node_t out;
	out.cdr.node = p;
	out.car.integer = 1;
	out.vtype = bug_ptr;
	return out;
}
bug_node_t bug_is_a(bug_node_t b, bug_type_t t){
	bool out = b.vtype == t;
	bug_node_t node;
	node.vtype = bug_bool;
	node.car.boolean = out;
	node.cdr.integer = 0;
	return node;
}
void runtime_checkups(bug_context_t * context){
	gc_collect(context->stack, context->stack_ptr, context->heap);
}
