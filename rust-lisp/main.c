#include "prelude.h"
extern bug_string bug_to_stringlong(long x0);
extern bug_string bug_to_stringdouble(double x0);
extern void bug_test();
extern void bug_test();
extern long bug_printlnbug_string(bug_string x0);
extern long bug_printbug_string(bug_string x0);
extern long bug_main();
void bug_test(){
    long x0;
    x0 = bug_printlnbug_string(to_bug_string("testing 1 2 3"));
}

long bug_main(){
    long x0;
    x0 = bug_printlnbug_string(to_bug_string("i love toast"));
    long x1;
    x1 = 2;
    bug_string x2;
    x2 = bug_to_stringlong(x1);
    long x3;
    x3 = bug_printlnbug_string(x2);
    bug_test();
    return 1;
}

int main(int argc ,const char ** argv){ return (int)(bug_main());}
