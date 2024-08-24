#pragma once
#include <stdio.h>
#include <stdlib.h>
#include "prog_builtins.h"
void gc_push_frame();
void gc_pop_frame();
void gc_push(void * ptr, void (*collect_fn)(void *));
void check_should_gc_collect();