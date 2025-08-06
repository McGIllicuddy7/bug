#include "prelude.h"
extern void bug_lamdba0(bug_context_t * in_context);
extern void bug_lamdba0(bug_context_t * in_context);
extern bug_node_t bug_printlnbug_string(bug_context_t * in_context);
extern bug_node_t bug_main(bug_context_t * in_context);
extern bug_node_t bug_main(bug_context_t * in_context);
extern bug_node_t bug_to_stringlong(bug_context_t * in_context);
extern bug_node_t bug_to_stringdouble(bug_context_t * in_context);
extern bug_node_t bug_printbug_string(bug_context_t * in_context);
extern bug_node_t bug_testbug_string(bug_context_t * in_context);
void bug_lamdba0(bug_context_t * in_context){
    bug_context_t context = *in_context;bug_context_t out_context = context;
    context.stack_ptr += 1;
    out_context = context;
    out_context.stack= out_context.stack_ptr;
    *out_context.stack_ptr = context.captures[0];out_context.stack_ptr++;
    context.stack[0] = bug_printlnbug_string(&out_context);
}

bug_node_t bug_main(bug_context_t * in_context){
    bug_context_t context = *in_context;bug_context_t out_context = context;
    context.stack_ptr += 5;
            context.stack[1] = bug_empty_list(&context);
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,to_bug_string(&context,"0")));
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,to_bug_string(&context,"1")));
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,to_bug_string(&context,"1")));
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,to_bug_string(&context,"2")));
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,to_bug_string(&context,"3")));
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,to_bug_string(&context,"5")));
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,to_bug_string(&context,"8")));
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,to_bug_string(&context,"13")));
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,to_bug_string(&context,"21")));
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,to_bug_string(&context,"34")));
    context.stack[1] = bug_list_cat(&context,context.stack[1],bug_box_value(&context,(bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 10}, .cdr= (bug_value_t){.integer =0}}));
    context.stack[0] = context.stack[1];
l0:
        if (context.stack[0].cdr.node) goto l1; else goto l2;
l1:
        if(bug_is_a(*(context.stack[0].cdr.node->car.node->cdr.node), bug_string).car.boolean) goto l3; else goto l4;l3:
            out_context = context;
            out_context.stack= out_context.stack_ptr;
            *out_context.stack_ptr = *(context.stack[0].cdr.node->car.node->cdr.node);out_context.stack_ptr++;
            context.stack[2] = bug_printlnbug_string(&out_context);
        goto l5;
l4:
            out_context = context;
            out_context.stack= out_context.stack_ptr;
            *out_context.stack_ptr = *(context.stack[0].cdr.node->car.node->cdr.node);out_context.stack_ptr++;
            context.stack[2] = bug_to_stringlong(&out_context);
            out_context = context;
            out_context.stack= out_context.stack_ptr;
            *out_context.stack_ptr = context.stack[2];out_context.stack_ptr++;
            context.stack[3] = bug_printlnbug_string(&out_context);
        goto l5;
l5:
        context.stack[0] = bug_cdr(context.stack[0]);
    goto l0;
l2:
        return (bug_node_t){.vtype = bug_integer, .car = (bug_value_t){.integer = 0}, .cdr= (bug_value_t){.integer =0}};
}

bug_node_t bug_testbug_string(bug_context_t * in_context){
    bug_context_t context = *in_context;bug_context_t out_context = context;
    context.stack_ptr += 1;
        context.stack[1] = (bug_node_t){.vtype = bug_void_fn,.car= (bug_value_t){.void_fn =bug_lamdba0},.cdr = {.ptr = bug_make_captures(&context,(int[]){0},1)}};
    return context.stack[1];
}

int main(int argc ,const char ** argv){ bug_context_t main_context = bug_create_context();int out =(int)(bug_main(&main_context).car.integer); gc_collect(main_context.stack, main_context.stack_ptr, main_context.heap); free_heap(&main_context); return out;}