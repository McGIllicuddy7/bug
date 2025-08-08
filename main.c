#include "prelude.h"
extern bug_node_t bug_printbug_string(bug_context_t * in_context);
extern bug_node_t bug_testbug_string(bug_context_t * in_context);
extern bug_node_t bug_to_stringlong(bug_context_t * in_context);
extern bug_node_t bug_to_stringdouble(bug_context_t * in_context);
extern void bug_lamdba0(bug_context_t * in_context);
extern void bug_lamdba0(bug_context_t * in_context);
extern bug_node_t bug_main(bug_context_t * in_context);
extern bug_node_t bug_main(bug_context_t * in_context);
extern bug_node_t bug_printlnbug_string(bug_context_t * in_context);
bug_node_t bug_testbug_string(bug_context_t * in_context){
    bug_context_t context = *in_context;bug_context_t out_context = context;
    bug_node_t *arg_prev = context.stack_ptr;
    context.stack_ptr += 1;
    memset(arg_prev, 0, sizeof(bug_node_t)*1);
        context.stack[1] = (bug_node_t){.vtype = bug_void_fn,.car= (bug_value_t){.void_fn =bug_lamdba0},.cdr = {.ptr = bug_make_captures(&context,(int[]){0},1)}};
    runtime_checkups(&context);
    return context.stack[1];
    runtime_checkups(&context);
}

void bug_lamdba0(bug_context_t * in_context){
    bug_context_t context = *in_context;bug_context_t out_context = context;
    bug_node_t *arg_prev = context.stack_ptr;
    context.stack_ptr += 1;
    memset(arg_prev, 0, sizeof(bug_node_t)*1);
    out_context = context;
    out_context.stack= out_context.stack_ptr;
    *out_context.stack_ptr = context.captures[0];out_context.stack_ptr++;
    context.stack[0] = bug_printlnbug_string(&out_context);
    runtime_checkups(&context);
}

bug_node_t bug_main(bug_context_t * in_context){
    bug_context_t context = *in_context;bug_context_t out_context = context;
    bug_node_t *arg_prev = context.stack_ptr;
    context.stack_ptr += 11;
    memset(arg_prev, 0, sizeof(bug_node_t)*11);
            context.stack[1] = bug_empty_list(&context);
    context.stack[0] = context.stack[1];
        context.stack[2] = (bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 0}, .cdr= (bug_value_t){.integer =0}};
l0:
            context.stack[3].car.boolean = context.stack[2].car.integer<=(bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 100}, .cdr= (bug_value_t){.integer =0}}.car.integer;    if (context.stack[3].car.boolean) goto l1; else goto l2;
l1:
        out_context = context;
        out_context.stack= out_context.stack_ptr;
        *out_context.stack_ptr = context.stack[2];out_context.stack_ptr++;
        context.stack[3] = bug_to_stringlong(&out_context);
        out_context = context;
        out_context.stack= out_context.stack_ptr;
        *out_context.stack_ptr = context.stack[3];out_context.stack_ptr++;
        context.stack[4] = bug_printlnbug_string(&out_context);
        context.stack[5].car.integer = context.stack[2].car.integer+(bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 1}, .cdr= (bug_value_t){.integer =0}}.car.integer;
        context.stack[2] = context.stack[5];
    runtime_checkups(&context);
    goto l0;
l2:
        context.stack[2] = (bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 0}, .cdr= (bug_value_t){.integer =0}};
l3:
            context.stack[3].car.boolean = context.stack[2].car.integer<=(bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 91}, .cdr= (bug_value_t){.integer =0}}.car.integer;    if (context.stack[3].car.boolean) goto l4; else goto l5;
l4:
        out_context = context;
        out_context.stack= out_context.stack_ptr;
        *out_context.stack_ptr = context.stack[2];out_context.stack_ptr++;
        context.stack[3] = bug_to_stringlong(&out_context);
        context.stack[0] = bug_list_cat(&context, context.stack[0], bug_box_value(&context,context.stack[3]));
        context.stack[4].car.integer = context.stack[2].car.integer+(bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 1}, .cdr= (bug_value_t){.integer =0}}.car.integer;
        context.stack[2] = context.stack[4];
    runtime_checkups(&context);
    goto l3;
l5:
    l6:
        if (context.stack[0].cdr.node) goto l7; else goto l8;
l7:
        out_context = context;
        out_context.stack= out_context.stack_ptr;
        *out_context.stack_ptr = *(context.stack[0].cdr.node->car.node->cdr.node);out_context.stack_ptr++;
        context.stack[3] = bug_printlnbug_string(&out_context);
        context.stack[0] = bug_cdr(context.stack[0]);
    runtime_checkups(&context);
    goto l6;
l8:
        runtime_checkups(&context);
    return (bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 0}, .cdr= (bug_value_t){.integer =0}};
    runtime_checkups(&context);
}

int main(int argc ,const char ** argv){ bug_context_t main_context = bug_create_context();int out =(int)(bug_main(&main_context).car.integer); gc_collect(main_context.stack, main_context.stack_ptr, main_context.heap); free_heap(&main_context); return out;}
