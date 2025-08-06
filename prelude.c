#include "prelude.h"
//every allocation either contains structured data in the form of bug_objects or just bytes (string)
void local_move_ptr(void * base_ptr, void * to_move_to, bug_node_t * node, bool is_root){
	if(!is_root){
		bug_allocation_t * alc =(void*)((char*)node-sizeof(bug_allocation_t));
		if(alc->reached_move){
			return;
		}else{
			alc->reached_move = true;
		}
	}
	if(node->vtype == bug_ptr){
		bug_node_t * nd = node->cdr.node;
		if(nd == base_ptr){
			node->car.node = to_move_to;
			nd = to_move_to;
		}
		for(long i =0; i<node->car.integer; i++){
			local_move_ptr(base_ptr, to_move_to,nd+i, false);
		}
	}
	if(node->vtype == bug_string){
		char * base = node->cdr.char_ptr;
		if(base == base_ptr){
			node->cdr.char_ptr = base;
		}
	}
	if(node->vtype == bug_void_fn || node->vtype == bug_non_void_fn){
		bug_node_t * base = node->cdr.node-1;
		if(node->cdr.node ==base_ptr){
			node->cdr.node = to_move_to;
			base = node->cdr.node-1; 
		}
		long l = base->car.integer;
		base += 1;
		for(long i =0 ;i<l; i++){
			local_move_ptr(base_ptr, to_move_to,base+i,false);
		}
	}
	if(node->vtype == bug_list_ptr){
		if(node->car.node == base_ptr){
			node->car.node = to_move_to;
		}
		if(node->cdr.node == base_ptr){
			node->cdr.node = to_move_to;
		}
		local_move_ptr(base_ptr, to_move_to,node->car.node,false);
		local_move_ptr(base_ptr, to_move_to,node->cdr.node,false);
	}
	if(node->vtype == bug_list_integer || node->vtype == bug_list_double || node->vtype == bug_list_char || node->vtype == bug_list_bool){	
		if(node->cdr.node == base_ptr){
			node->cdr.node = to_move_to;
		}	
		local_move_ptr(base_ptr, to_move_to,node->cdr.node,false);
	}
}
void move_pointer(void * base_ptr, void * to_move_to, bug_node_t * start, bug_node_t * end, bug_heap_t * heap){
	bug_allocation_t * cur = heap->gen1;
	while(cur){	
		cur = cur->next;
		cur->reached_move = false;
	}
	cur = heap->allocations;
	while(cur){
		cur = cur->next;
		cur->reached_move = false;
	}
	for(bug_node_t * b = start; b != end; b++){
		local_move_ptr(base_ptr, to_move_to, b,  true);		
	}
}
void mark_reachable_from(bug_node_t * node, bool is_root){
	if(!is_root){	
		bug_allocation_t * alloc = (bug_allocation_t*)((char*)node-16);
		if(alloc->is_reachable){
			return;
		}
		alloc->is_reachable= true;
	}
	if(node->vtype == bug_ptr){
		bug_node_t * nd = node->cdr.node;
		for(long i =0; i<node->car.integer; i++){
			mark_reachable_from(nd+i, false);
		}
	}
	if(node->vtype == bug_string){
		char * base = node->cdr.char_ptr;
		bug_allocation_t * b = (bug_allocation_t*)(base-sizeof(bug_allocation_t));
		b->is_reachable = true;
	}
	if(node->vtype == bug_void_fn || node->vtype == bug_non_void_fn){
		bug_node_t * base = node->cdr.node-1;
		long l = base->car.integer;
		base += 1;
		for(long i =0 ;i<l; i++){
			mark_reachable_from(base+i,false);
		}
	}
	if(node->vtype == bug_list_ptr){
		mark_reachable_from(node->car.node,false);
		mark_reachable_from(node->cdr.node,false);
	}
	if(node->vtype == bug_list_integer || node->vtype == bug_list_double || node->vtype == bug_list_char || node->vtype == bug_list_bool){	
		mark_reachable_from(node->cdr.node,false);
	}
}
void gc_collect(bug_node_t * base, bug_node_t *end, bug_heap_t * heap){
	bug_allocation_t * cur = heap->allocations;	
	while(cur){
		cur->is_reachable = false;
		cur = cur->next;
	}
	for(bug_node_t * node = base; node != end; node++){
		mark_reachable_from(node,true);
	}
	cur = heap->gen1;
	while(cur){
		if(cur->is_reachable){
			void * s = malloc(cur->byte_count);
			memcpy(s, cur, cur->byte_count);
			move_pointer((char*)cur+sizeof(bug_allocation_t),(char*)s+sizeof(bug_allocation_t), base, end, heap);
			((bug_allocation_t*)s)->next = heap->allocations;
			heap->allocations = s;
		} else {
//			printf("allocation from gen1 :%p is unreachable\n", cur);
		}
		cur = cur->next;
	}
	cur = heap->allocations;
	bug_allocation_t *old =0;
	while(cur){
		bug_allocation_t * next = cur->next;
		if(!cur->is_reachable){
			if(old){
				old->next = next;
			}else{
				heap->allocations = next;
			}
			free(cur);
		}else{
//		printf("allocation from heap :%p is unreachable\n", cur);
			old = cur;
		}
		cur = next;
	}
}

bug_context_t bug_create_context(){
	bug_context_t out;
	out.stack = (bug_node_t*)malloc(sizeof(bug_node_t)*16000);
	out.base_ptr = out.stack;
	out.stack_ptr = out.stack;
	out.stack_end = out.stack+16000;
	out.heap = malloc(sizeof(bug_heap_t));
	out.heap->allocations =0;
	out.heap->gen1 =0;
	out.heap->gen1_heap =malloc(16000);
	out.heap->gen1_next = out.heap->gen1_heap;
	out.heap->gen1_heap_end = out.heap->gen1_heap+16000;
	return out;
}
bug_context_t bug_reserve_stack_space(bug_context_t * context,size_t object_count);

void * gc_alloc(bug_context_t* context, size_t count){
	if(count>=512){
		void * out;
	just_return:
		out = malloc(count+sizeof(bug_allocation_t));
		bug_allocation_t * alc = out;
		alc->byte_count = count+sizeof(bug_allocation_t);
		alc->next = context->heap->allocations;
		context->heap->allocations = alc;
		return (char*)out + sizeof(bug_allocation_t);
	}else{
		
		void * out =(void*)context->heap->gen1_next;
		size_t offset;
		if (count %16 != 0){offset = count+16-count%16;} 
		else{offset =count;}
		offset += sizeof(bug_allocation_t);
		if(context->heap->gen1_next+offset >= context->heap->gen1_heap_end){
			gc_collect(context->base_ptr, context->stack_ptr, context->heap);
			if(context->heap->gen1_next+offset >= context->heap->gen1_heap_end){
				goto just_return;
			}
		}
		context->heap->gen1_next+= offset;
		bug_allocation_t * alc = out;
		alc->next = context->heap->gen1;
		alc->byte_count = offset;
		alc->is_reachable = false;
		context->heap->gen1 = alc;
		return (char*)out+sizeof(bug_allocation_t);
	}
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
	bug_node_t * out = (bug_node_t*)gc_alloc(context,sizeof(bug_node_t)*(count+1));
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
	free(context->heap->gen1_heap);
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
	}
	if(node->vtype == bug_integer){
		printf("%ld" ,node->car.integer);
	}
	if(node->vtype == bug_double){
		printf("%f" ,node->car.db);
	}
	if(node->vtype == bug_char){
		printf("%c" ,node->car.character);
	}
	if(node->vtype == bug_bool){
		printf("%b" ,node->car.boolean);
	}
	if(node->vtype == bug_string){
		printf("\"%.*s\"" ,(int)(node->car.integer), node->cdr.char_ptr);
	}
	if(node->vtype == bug_list_ptr ){ 
		debug_print_node(node->car.node, indentation+1);
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	if(node->vtype == bug_list_ptr ){ 
		debug_print_node(node->car.node, indentation+1);
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	if(node->vtype == bug_list_integer){ 
		printf("%ld\n", node->car.integer);	
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	if(node->vtype == bug_list_double){ 
		printf("%f\n", node->car.db);	
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	if(node->vtype == bug_list_char){ 
		printf("%c\n", node->car.character);	
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	if(node->vtype == bug_list_bool){ 
		printf("%b\n", node->car.boolean);	
		debug_print_node(node->cdr.node, indentation+1);
		for(int i =0; i<indentation; i++){
			printf("   ");
		}	
	}	
	printf("}\n");


}
void debug_node(bug_node_t * node){
	debug_print_node(node,0);
}
bug_node_t bug_empty_list(bug_context_t * context){
	bug_node_t out;
	out.vtype = bug_ptr;;
	out.car.integer =0;
	out.cdr.ptr= 0;
	return out;
}
bug_node_t bug_list_cat(bug_context_t * context, bug_node_t base, bug_node_t end){
	bug_node_t * out = (bug_node_t*)gc_alloc(context, sizeof(bug_node_t));
	bug_node_t * box =(bug_node_t*)gc_alloc(context, sizeof(bug_node_t));
	*box = end;
	out->cdr.node =0;
	out->car.node = box;
	if(!base.cdr.node){
		base.cdr.node = out;
		return base;
	}
	bug_node_t *cur = base.cdr.node;
	while(true){
		if(!cur->cdr.node){
			cur->cdr.node = out;
			break;
		}
		cur = cur->cdr.node;
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
	bug_node_t * p = (bug_node_t*)gc_alloc(context, sizeof(node));
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
