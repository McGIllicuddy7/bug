#include "builtins.h"
#include <unistd.h>
#include <string.h>
#include <assert.h>
#include <sys/mman.h>
#include <stdbool.h>
#define BUFFER_ALLOCATION_COUNT 512
#define SMOL_SIZE_LEN 128
typedef struct{void * ptr;size_t size;}gc_allocation;
typedef struct {size_t reachable;}allocation_header;
typedef struct allocation_buffer{
    gc_allocation allocations[BUFFER_ALLOCATION_COUNT];
    struct allocation_buffer * next;
    struct allocation_buffer * prev;
}allocation_buffer;
typedef struct {void * ptr; void (*mark)(void *);}gc_ptr;
typedef struct gc_frame{
    gc_ptr *buffer;
    size_t next_ptr;
    size_t sz;
    gc_ptr smol_size[SMOL_SIZE_LEN];
    struct gc_frame * next;
    struct gc_frame * prev;
}gc_frame;

static gc_frame * current_frame = 0;
static allocation_buffer allocations = {0};
static ssize_t allocation_count = 0;
static size_t dropped_ptr_count = 0;
static void * stack_min = 0;
static void * stack_max = 0;
void * mem_alloc(size_t size){
    allocation_count++;
    void * out = calloc(size,1);
    return out;
}
void mem_free(void * to_free){
    allocation_count--;
    free(to_free);
}
void gc_push_frame(){
    gc_frame * nw = malloc(sizeof(gc_frame));
    nw->buffer = nw->smol_size;
    nw->next = 0;
    nw->next_ptr = 0;
    nw->sz = SMOL_SIZE_LEN;
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
    if(current_frame){
        stack_min = current_frame->buffer[current_frame->next_ptr-1].ptr;
    } else{
        stack_min = 0;
    }

    gc_collect();
}
void gc_register_ptr(void * ptr, void (*collect_fn)(void *)){
    if (stack_max == 0){
        stack_max = ptr;
    }
    if(stack_min == 0){
        stack_min = ptr;
    }
    if(ptr>stack_max){
        stack_max = ptr;
    }
    if(ptr<stack_min){
        stack_min = ptr;
    }
    if(current_frame->next_ptr>=current_frame->sz){
        current_frame->sz *=2;
        if(current_frame->buffer == current_frame->smol_size){
            current_frame->buffer = malloc(sizeof(gc_ptr)*current_frame->sz);
            memcpy(current_frame->buffer, current_frame->smol_size, sizeof(gc_ptr)*SMOL_SIZE_LEN);
        }else{
            current_frame->buffer = realloc(current_frame->buffer, sizeof(gc_ptr)*current_frame->sz);
        }
    }
    current_frame->buffer[current_frame->next_ptr] =(gc_ptr){ptr, collect_fn};
    current_frame->next_ptr ++;
}

gc_allocation * find_allocation(allocation_buffer * buffer){
    for(int i =0; i<BUFFER_ALLOCATION_COUNT; i++){
        if (buffer->allocations[i].ptr == 0){
            return &buffer->allocations[i];
        }
    }
    if (!buffer->next){
        buffer->next = calloc(sizeof(allocation_buffer),1);
        buffer->next->prev = buffer;
        return find_allocation(buffer->next);
    } else{
        return find_allocation(buffer->next);
    }
}
void gc_collect(){
    if(dropped_ptr_count <64 && current_frame != 0){
        return;
    }
    dropped_ptr_count = 0;
    gc_frame * current = current_frame;
    while(current){
        for(int i =0; i<current->next_ptr; i++){
            gc_ptr tmp = current->buffer[i];
            tmp.mark(tmp.ptr);
        }
        current = current->prev;
    }
    allocation_buffer * cur = &allocations;
    size_t allocation_count = 0;
    size_t byte_count =0;
    while(cur){
        for(int i =0; i<BUFFER_ALLOCATION_COUNT; i++){
            gc_allocation * a = &cur->allocations[i];
            if(a->ptr){
                allocation_header *al = a->ptr;
                if(!al->reachable){
                    allocation_count +=1;
                    byte_count += a->size;
                    void * ptr = a->ptr;
                    a->ptr = 0;
                    a->size = 0;
                    mem_free(ptr);
                } else{
                    al->reachable = false;
                }
            }
        }
        cur = cur->next;
    }
    if(allocation_count>0){
        printf("collected %zu bytes in %zu allocations\n", byte_count, allocation_count);
    }

}

void * gc_alloc(size_t size){
    if (size<8){
        size = 8;
    }
    allocation_header * base = mem_alloc(size+sizeof(allocation_header));
    base->reachable = 0;
    void * out = &base[1];
    gc_allocation alc = {base, size};
    //printf("allocated %p\n", out); 
    *find_allocation(&allocations) = alc;
    return out;
}


bool gc_any_ptr(void * ptr){
    if(ptr == 0){
        return true;
    }
    if(ptr<=stack_max && ptr>=stack_min){
        return true;
    }
    allocation_header * base = ptr;
    if(base[-1].reachable){
        return true;
    }
    base[-1].reachable = true;
    return false;
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
    char * out = gc_alloc(len);
    memcpy(out, str, len);
    return (String){out, len};
}
size_t get_allocation_count(){
    printf("%zd remaining allocations", allocation_count);
    return allocation_count;
}
long user_mod_long_long(long a, long b){
    return a%b;
}
void user_put_str_String(String s){
    write(1, s.start, s.len);
}
extern long user_main();
int main(int argc,const char ** argv){
        long result = user_main();
        printf("exited with %ld\n",result);
        gc_collect(); 
        assert(get_allocation_count() == 0);
        return result;
}