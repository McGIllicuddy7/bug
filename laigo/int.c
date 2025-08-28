#include <stdio.h>
#include <stdlib.h>
typedef union {
    size_t sz;
    void * ptr;
    long long lng;
}reg_t;
size_t interupt(size_t cd, reg_t x0, reg_t x1 ){
    if(cd == 1){
        printf("%lld\n", x0.lng);
        exit(0);
    }
    return 0;
}
