#include "prelude.h"
extern bug_node_t bug_main(bug_context_t * in_context);
extern bug_node_t bug_main(bug_context_t * in_context);
extern bug_node_t bug_testbug_string(bug_context_t * in_context);
extern bug_node_t bug_printlnbug_string(bug_context_t * in_context);
extern bug_node_t bug_to_stringlong(bug_context_t * in_context);
extern bug_node_t bug_to_stringdouble(bug_context_t * in_context);
extern bug_node_t bug_printbug_string(bug_context_t * in_context);
extern void bug_lamdba0(bug_context_t * in_context);
extern void bug_lamdba0(bug_context_t * in_context);
bug_node_t bug_main(bug_context_t * in_context){
    bug_context_t context = *in_context;bug_context_t out_context = context;
    while(context.stack[5].car.boolean){        context.stack[5].car.boolean = context.stack[4].car.integer<=(bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 1000}, .cdr= (bug_value_t){.integer =0}}.car.integer        out_context = context;
        out_context.stack= out_context.stack_ptr;
        *out_context.stack_ptr = context.stack[4];out_context.stack_ptr++;
        context.stack[6] = bug_to_stringlong(&context);
        out_context = context;
        out_context.stack= out_context.stack_ptr;
        *out_context.stack_ptr = context.stack[6];out_context.stack_ptr++;
        context.stack[7] = bug_printlnbug_string(&context);
        context.stack[8].car.integer = context.stack[4].car.integer+(bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 1}, .cdr= (bug_value_t){.integer =0}}.car.integer;
        context.stack[4] = context.stack[8];
    }
    return (bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 0}, .cdr= (bug_value_t){.integer =0}};
}

bug_node_t bug_testbug_string(bug_context_t * in_context){
    bug_context_t context = *in_context;bug_context_t out_context = context;
        context.stack[1] = (bug_node_t){.vtype = bug_void_fn,.car= (bug_value_t){.void_fn =bug_lamdba0},.cdr = {.ptr = bug_make_captures(&context,(int[]){0},1)}};
    return context.stack[1];
}

void bug_lamdba0(bug_context_t * in_context){
    bug_context_t context = *in_context;bug_context_t out_context = context;
    out_context = context;
    out_context.stack= out_context.stack_ptr;
    *out_context.stack_ptr = context.captures[0];out_context.stack_ptr++;
    context.stack[0] = bug_printlnbug_string(&context);
}

int main(int argc ,const char ** argv){ bug_context_t main_context = bug_create_context();int out =(int)(bug_main(&main_context).car.integer); gc_collect(main_context.stack, main_context.stack_ptr, main_context.heap); free_heap(&main_context); return out;}