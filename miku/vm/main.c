#include "vm.h"
#include "asm.h"
#include <stdio.h>
#define CTILS_STATIC [[maybe_unused]]
#define CTILS_IMPLEMENTATION
#include "../utils.h"
int main(int argc, const char ** argv){
	Arena * arena = arena_create();
	if(argc <2) return 1;
	vm_t* vm = compile_string(read_file_to_string(arena,argv[1]).items);
	while(run_instruction(vm)){
	}
}
