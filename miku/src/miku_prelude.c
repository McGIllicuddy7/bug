#include "miku_prelude.h"
#include <stdio.h>
extern long miku_main();
int main(int argc, const char ** argv){
    printf("%ld\n",miku_main());
}
