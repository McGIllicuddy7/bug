#include "builtins.h"
#include "unistd.h"
#include <string.h>
#include <assert.h>
typedef struct{void * ptr;size_t size; bool reachable;}gc_allocation;
typedef struct allocation_buffer{
    gc_allocation allocations[1024];
    struct allocation_buffer * next;
    struct allocation_buffer * prev;
}allocation_buffer;
static allocation_buffer allocations = {0};
typedef struct gc_frame{
    struct gc_frame * next;
    struct gc_frame * prev;
}gc_frame;
gc_allocation * find_allocation(allocation_buffer * buffer){
    for(int i =0; i<1024; i++){
        if (buffer->allocations[i].ptr == 0){
            return &buffer->allocations[i];
        }
    }
    if (!buffer->next){
        buffer->next = malloc(sizeof(allocation_buffer));
        buffer->next->prev = buffer;
        return find_allocation(buffer->next);
    } else{
        return find_allocation(buffer->next);
    }
}
void user_put_str_String(String s){
    write(1, s.start, s.len);
}
void * gc_alloc(size_t size){
    void * out = calloc(size,1);
    gc_allocation alc = {out, size,0};
    *find_allocation(&allocations) = alc;
    return out;
}
void gc_push_frame(){
    
}
void gc_pop_frame(){

}
void gc_register_ptr(void * ptr, void (*collect_fn)(void *)){

}
void check_should_gc_collect(){

}
void gc_any_ptr(void * ptr){

}
void gc_long(void * ptr){

}
void gc_String(void * ptr){

}
void gc_double(void * ptr){

}
void gc_char(void * ptr){

}
void gc_bool(void * ptr){
    
}
String operator_plus_String_String(String a, String b){
    size_t out_l = a.len+b.len;
    char * out_buff = gc_alloc(out_l);
    memcpy(out_buff, a.start, a.len);
    memcpy(out_buff+a.len, b.start, b.len);
    String out =  (String){out_buff, out_l};

    return out;
}
String user_int_to_string_long(long a){
    char buffer[100] = {0};
    snprintf(buffer, 99, "%ld", a);
    size_t l = strlen(buffer);
    char * out = gc_alloc(l);
    memcpy(out, buffer, l);
    return (String){out, l};
}
String make_string_from(const char * str, size_t len){
    char * out = gc_alloc(len);
    memcpy(out, str, len);
    return (String){out, len};
}