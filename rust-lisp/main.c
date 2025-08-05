#include "prelude.h"
extern bug_node_t bug_printlnbug_string(bug_context_t * in_context);
extern bug_node_t bug_to_stringlong(bug_context_t * in_context);
extern bug_node_t bug_to_stringdouble(bug_context_t * in_context);
extern bug_node_t bug_testbug_string(bug_context_t * in_context);
extern void bug_lamdba0(bug_context_t * in_context);
extern void bug_lamdba0(bug_context_t * in_context);
extern bug_node_t bug_main(bug_context_t * in_context);
extern bug_node_t bug_main(bug_context_t * in_context);
extern bug_node_t bug_printbug_string(bug_context_t * in_context);
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

bug_node_t bug_main(bug_context_t * in_context){
    bug_context_t context = *in_context;bug_context_t out_context = context;
    out_context = context;
    out_context.stack= out_context.stack_ptr;
    *out_context.stack_ptr = to_bug_string(&context,"i have working lambdas");out_context.stack_ptr++;
    context.stack[0] = bug_printlnbug_string(&context);
        out_context = context;
    out_context.stack= out_context.stack_ptr;
    *out_context.stack_ptr = to_bug_string(&context,"also i love you");out_context.stack_ptr++;
    context.stack[2] = bug_testbug_string(&context);
    context.stack[1] = context.stack[2];
out_context = context;
    out_context.stack= out_context.stack_ptr;out_context.captures = context.stack[1].cdr.node;
    (context.stack[1].car.void_fn)(&out_context);
    return (bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 0}, .cdr= (bug_value_t){.integer =0}};
}

int main(int argc ,const char ** argv){ bug_context_t main_context = bug_create_context();return (int)(bug_main(&main_context).car.integer);}