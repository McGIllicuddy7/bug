#include "prog_builtins.h"
#include "unistd.h"
void user_put_str_String(String s){
    write(1, s.start, s.len);
}
void gc_push_frame(){
    
}
void gc_pop_frame(){

}
void gc_push(void * ptr, void (*collect_fn)(void *)){

}
void check_should_gc_collect(){

}