#include "builtins.h"
#include <unistd.h>
#include <string.h>
#include <assert.h>
#include <sys/mman.h>
#include <stdbool.h>
#define false 0
#define true 1
static ssize_t allocation_count = 0;
static size_t dropped_ptr_count = 0;
#define BUFFER_ALLOCATION_COUNT 512
void * mem_alloc(size_t size){
    allocation_count++;
    return calloc(size,1);
}
void mem_free(void * to_free){
    allocation_count--;
    free(to_free);
}
typedef struct{void * ptr;size_t size;}gc_allocation;
typedef struct {size_t reachable;}allocation_header;
typedef struct {void * ptr; void (*mark)(void *);}gc_ptr;
typedef struct gc_frame{
    gc_ptr *buffer;
    size_t next_ptr;
    size_t sz;
    gc_ptr smol_size[128];
    struct gc_frame * next;
    struct gc_frame * prev;
}gc_frame;
static gc_frame * current_frame = 0;
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
    dropped_ptr_count += prev->next_ptr;
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
void gc_collect(){
    
}
void * gc_alloc(size_t size){
    if (size<8){
        size = 8;
    }
    allocation_header * base = mem_alloc(size+sizeof(allocation_header));
    base->reachable = 0;
    void * out = &base[1];
    gc_allocation alc = {out, size};
    //printf("allocated %p\n", out); 
    return out;
}


bool gc_any_ptr(void * ptr){
    assert(false);
}
void gc_long(void * ptr){

}
void gc_String(void * ptr){
    String s= *(String *)(ptr);
    gc_any_ptr((void *)s.start);
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
    const char * out = gc_alloc(len);
    memcpy(out, str, len);
    return (String){out, len};
}
ssize_t get_allocation_count(){
    return allocation_count;
}
long user_mod_long_long(long a, long b){
    return a%b;
}
void user_put_str_String(String s){
    write(1, s.start, s.len);
}