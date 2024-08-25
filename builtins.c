#include "builtins.h"
#include <unistd.h>
#include <string.h>
#include <assert.h>
#include <sys/mman.h>
#include <stdbool.h>
#define false 0
#define true 1
static ssize_t allocation_count = 0;
void * mem_alloc(size_t size){
    allocation_count++;
    return malloc(size);
}
void mem_free(void * to_free){
    allocation_count--;
    free(to_free);
}
typedef struct{void * ptr;size_t size; bool reachable;}gc_allocation;
typedef struct allocation_buffer{
    gc_allocation allocations[1024];
    struct allocation_buffer * next;
    struct allocation_buffer * prev;
}allocation_buffer;
typedef struct {void * ptr; void (*mark)(void *);}gc_ptr;
typedef struct gc_frame{
    gc_ptr *buffer;
    size_t next_ptr;
    size_t sz;
    gc_ptr smol_size[128];
    struct gc_frame * next;
    struct gc_frame * prev;
}gc_frame;

static allocation_buffer allocations = {0};
static gc_frame * current_frame = 0;
gc_allocation * find_allocation(allocation_buffer * buffer){
    for(int i =0; i<1024; i++){
        if (buffer->allocations[i].ptr == 0){
            return &buffer->allocations[i];
        }
    }
    if (!buffer->next){
        buffer->next = calloc(sizeof(allocation_buffer),0);
        buffer->next->prev = buffer;
        return find_allocation(buffer->next);
    } else{
        return find_allocation(buffer->next);
    }
}
void user_put_str_String(String s){
    write(1, s.start, s.len);
}
void gc_collect(){
    static int counter = 0;
    counter = 0;
    gc_frame * current = current_frame;
    while(current){
        for(int i =0; i<current->next_ptr; i++){
            gc_ptr tmp = current->buffer[i];
            tmp.mark(tmp.ptr);
        }
        current = current->prev;
    }
    allocation_buffer * cur = &allocations;
    while(cur){
        for(int i =0; i<1024; i++){
            gc_allocation * a = &cur->allocations[i];
            if(!a->reachable && a->ptr){
                //printf("mem_freed %p\n", a->ptr);
		    mem_free(a->ptr);
                a->ptr = 0;
                a->size = 0;
            }
            a->reachable = false;
        }
        cur = cur->next;
    }
}

void * gc_alloc(size_t size){
    if (size<16){
        size = 16;
    }
    void * out = mem_alloc(size);
    gc_allocation alc = {out, size,1};
    //printf("allocated %p\n", out); 
    *find_allocation(&allocations) = alc;
    return out;
}
void gc_push_frame(){
    gc_frame * nw = malloc(sizeof(gc_frame));
    nw->buffer = nw->smol_size;
    nw->next = 0;
    nw->next_ptr = 0;
    nw->sz = 128;
    nw->prev = current_frame;
    if(current_frame){
        current_frame->next = nw;
    }
    current_frame = nw;
}
void gc_pop_frame(){
    gc_frame * prev = current_frame;
    if(prev->buffer != prev->smol_size){
        free(prev->buffer);
    }
    current_frame = prev->prev;
    free(prev);
    gc_collect();
}
void gc_register_ptr(void * ptr, void (*collect_fn)(void *)){
    if(current_frame->next_ptr>current_frame->sz){
        current_frame->sz *=2;
        if(current_frame->buffer == current_frame->smol_size){
            current_frame->buffer = malloc(sizeof(gc_ptr)*current_frame->sz);
            memcpy(current_frame->buffer, current_frame->smol_size, sizeof(gc_ptr)*128);
        }else{
            current_frame->buffer = realloc(current_frame->buffer, sizeof(gc_ptr)*current_frame->sz);
        }
    }
    current_frame->buffer[current_frame->next_ptr] =(gc_ptr){ptr, collect_fn};
    current_frame->next_ptr ++;
}

void gc_any_ptr(void * ptr){
    allocation_buffer *current =  &allocations;
    while(current){
        for(int i =0; i<1024; i++){
            gc_allocation * a = &current->allocations[i];
            if (ptr>=a->ptr && ptr<a->ptr+a->size){
                a->reachable = true;
            }
        }
        current = current->next;
    }
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
    const char * out = str;
    return (String){out, len};
}
ssize_t get_allocation_count(){
    return allocation_count;
}