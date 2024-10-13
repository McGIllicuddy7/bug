#include <stdio.h>
#include "arena.h"

int main(){
    Arena a = Arena();
    for(int i =0; i<10; i++){
       DEFER(a, printf("%d\n",i), i);
    }
    printf("moist\n");
}
