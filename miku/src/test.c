#include "miku_prelude.h"



long miku_main();
long miku_test(long a);

long miku_main(){
    long a;
    long b;
    a = 10;
    b = 5;
    a = b + a;
    a = miku_test(10);
    return a;
}
long miku_test(long a){
    a = 10 + a;
    return a;
}
