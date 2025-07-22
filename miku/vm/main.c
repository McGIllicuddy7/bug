#include "vm.h"
#include "asm.h"
#include <stdio.h>
#define CTILS_STATIC [[maybe_unused]]
#define CTILS_IMPLEMENTATION
#include "../utils.h"
int main(int argc, const char ** argv){
	Arena * arena = arena_create();
	const char * to_run = "main.s";
	if(argc >1) to_run = argv[1];
	vm_t* vm = compile_string(read_file_to_string(arena,to_run).items);
	while(run_instruction(vm)){
//		debug_vm(vm);
//		sleep(1);
	}
}
