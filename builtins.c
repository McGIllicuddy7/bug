#include "builtins.h"
#include "unistd.h"
#include <string.h>
#include <assert.h>
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