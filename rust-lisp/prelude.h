#pragma once 
#include "stdint.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
typedef struct{
	const char * items;
	size_t len;
}bug_string;
#define to_bug_string(s)(bug_string){.items= s, .len= sizeof(s)}
static inline long bug_printbug_string(bug_string s){
	return printf("%.*s", (int)s.len, s.items);
}
static inline long bug_printlnbug_string(bug_string s){
	return printf("%.*s\n", (int)s.len, s.items);
}
static inline bug_string bug_to_stringlong(long v){
	char buffer[100];
	snprintf(buffer, 99, "%ld", v);
	size_t l= strlen(buffer);
	char *out = (char*)malloc(l);
	memcpy(out, buffer, l);
	return (bug_string){.items = out, .len = l};
}
static inline bug_string bug_to_stringdouble(double v){
	char buffer[100];
	snprintf(buffer, 99, "%f", v);
	size_t l= strlen(buffer);
	char *out = (char*)malloc(l);
	memcpy(out, buffer, l);
	return (bug_string){.items = out, .len = l};
}
