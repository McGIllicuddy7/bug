#include "prog_builtins.h"
#include "unistd.h"
void user_put_str_String(String s){
    write(1, s.start, s.len);
}