#include "builtins.h"
#include "unistd.h"

void user_put_str_String(String s){
    write(1, s.start, s.len);
}
void * gc_alloc(size_t size){
    return calloc(size,1);
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