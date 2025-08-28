#include <stdio.h>
#include <stdlib.h>
typedef union {
    size_t sz;
    void * ptr;
    long long lng;
}reg_t;
size_t interupt(size_t cd, reg_t x0, reg_t x1 ){
    if(cd == 0){
        printf("%lld\n", x0.lng);
    }
    return 0;
}
