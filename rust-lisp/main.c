#include "prelude.h"
extern long bug_main();
extern long bug_addlonglong(long _x0,long _x1);
extern void bug_test();
extern long bug_printlnbug_string(bug_string _x0);
extern bug_string bug_to_stringlong(long _x0);
extern bug_string bug_to_stringdouble(double _x0);
extern long bug_printbug_string(bug_string _x0);
long bug_main(){
    long _x0;
    _x0 = bug_printlnbug_string(to_bug_string("i love toast"));
    long _x1;
    long _x2;
    _x2 = bug_addlonglong(1,2);
    return _x2;
}

long bug_addlonglong(long _x0,long _x1){
    long _x4;
    _x4 = _x2+_x3;
    return _x4;
}

void bug_test(){
    long _x0;
    _x0 = bug_printlnbug_string(to_bug_string("testing 1 2 3"));
}

int main(int argc ,const char ** argv){ return (int)(bug_main());}
