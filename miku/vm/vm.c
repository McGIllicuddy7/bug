#include "vm.h"
#include <stdio.h>
#include "../utils.h"
#define STACK_VALUE(vm, ins)vm->memory[vm->registers[SP].word+ins.offset]
#define REGISTER1(vm, ins) vm->registers[ins.register1]
#define REGISTER2(vm, ins) vm->registers[ins.register2]
#define WORD(x) *(memory_t *)&(x)
void store_registers(vm_t * vm){
	for(int i =0; i<16; i++){
		printf("saving %s to %u\n", register_names[i], vm->registers[SP].word);	
		vm->registers[SP].word+= sizeof(memory_t);
	}
}
void load_registers(vm_t * vm){
	size_t sp = vm->registers[SP].word;
	sp-= sizeof(memory_t);
	for(int i =15; i>= 0; i--){
		printf("loading %s from %zu\n", register_names[i], sp);
		vm->registers[i] = WORD(vm->memory[sp]);
		sp-=sizeof(memory_t);
	}
}
void in_load(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins) = WORD(STACK_VALUE(vm, ins));
}
void in_load_abs(instruction_t ins, vm_t * vm){
	int r1 = ins.register1;
	vm->registers[IP].word++;
	uint32_t p = vm->instructions[vm->registers[IP].word].data;
	vm->registers[r1] = WORD(vm->memory[p]);
}

void in_store(instruction_t ins, vm_t * vm){
	WORD(STACK_VALUE(vm, ins)) = REGISTER1(vm, ins);
}

void in_store_abs(instruction_t ins, vm_t * vm){
	int r1 = ins.register1;
	vm->registers[IP].word++;
	uint32_t p = vm->instructions[vm->registers[IP].word].data;
	WORD(vm->memory[p])= vm->registers[r1] ;
}
void in_load_pointer(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins)= WORD(vm->memory[REGISTER2(vm, ins).word+ins.offset]);
}
void in_store_pointer(instruction_t ins, vm_t * vm){
	WORD(vm->memory[REGISTER1(vm, ins).word+ins.offset])= REGISTER2(vm, ins);
}
void in_call(instruction_t ins, vm_t * vm){	
	store_registers(vm);
	vm->registers[IP].word = vm->instructions[vm->registers[IP].word+1].data;
	vm->registers[BP] = vm->registers[SP];
}
void in_ret(instruction_t ins, vm_t * vm){
	vm->registers[SP] = vm->registers[BP];
	load_registers(vm);
	printf("new address %u:\n", vm->registers[IP].word);
}
void in_jmp(instruction_t ins, vm_t *vm){
	vm->registers[IP].word = vm->instructions[vm->registers[IP].word+1].data;	
}
void in_jmp_register(instruction_t ins, vm_t *vm){
	vm->registers[IP].word = REGISTER1(vm, ins).word;
}
void in_conditional_jmp(instruction_t ins, vm_t *vm){
	if(REGISTER1(vm, ins).word){
		vm->registers[IP].word = vm->instructions[vm->registers[IP].word+1].data;
	}
}
void in_conditional_jmp_register(instruction_t ins, vm_t *vm){
	if(REGISTER1(vm, ins).word){
		vm->registers[IP].word = REGISTER2(vm,ins).word;
	}
}

void in_mov(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins) = REGISTER2(vm, ins);
}
void in_mov_imm(instruction_t ins, vm_t * vm){
	vm->registers[IP].word++;
	REGISTER1(vm, ins).word = vm->instructions[vm->registers[IP].word].data;
}
void in_binop_add(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins).word+= REGISTER2(vm, ins).word;
}
void in_binop_subtract(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins).word-= REGISTER2(vm, ins).word;
}
void in_binop_multiply(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins).word *= REGISTER2(vm, ins).word;
}
void in_binop_divide(instruction_t ins, vm_t * vm){
	uint32_t r1 = REGISTER1(vm, ins).word;
	uint32_t r2 = REGISTER2(vm, ins).word;
	REGISTER1(vm, ins).word = r1/r2;
	REGISTER2(vm, ins).word = r1%r2;
}
void in_binop_compare_g(instruction_t ins, vm_t * vm){
	vm->registers[X].word= REGISTER1(vm, ins).integer>REGISTER2(vm, ins).integer;
}
void in_binop_compare_e(instruction_t ins, vm_t * vm){
	vm->registers[X].word= REGISTER1(vm, ins).integer==REGISTER2(vm, ins).integer;
}
void in_binop_compare_l(instruction_t ins, vm_t * vm){
	vm->registers[X].word= REGISTER1(vm, ins).integer<REGISTER2(vm, ins).integer;
}
void in_binop_unsigned_compare_g(instruction_t ins, vm_t * vm){
	vm->registers[X].word= REGISTER1(vm, ins).word>REGISTER2(vm, ins).word;
}
void in_binop_unsigned_compare_e(instruction_t ins, vm_t * vm){
	vm->registers[X].word= REGISTER1(vm, ins).word == REGISTER2(vm, ins).word;
}
void in_binop_unsigned_compare_l(instruction_t ins, vm_t * vm){
	vm->registers[X].word= REGISTER1(vm, ins).word<REGISTER2(vm, ins).word;
}
void in_binop_fp_add(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins).floating_point += REGISTER2(vm, ins).floating_point;
}
void in_binop_fp_subtract(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins).floating_point -= REGISTER2(vm, ins).floating_point;
}
void in_binop_fp_divide(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins).floating_point /= REGISTER2(vm,ins).floating_point;
}
void in_binop_fp_multiply(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins).floating_point *= REGISTER2(vm, ins).floating_point;
}
void in_binop_fp_compare_g(instruction_t ins, vm_t * vm){
	vm->registers[X].word= REGISTER1(vm, ins).floating_point>REGISTER2(vm, ins).floating_point;
}
void in_binop_fp_compare_e(instruction_t ins, vm_t * vm){
	vm->registers[X].word= REGISTER1(vm, ins).floating_point==REGISTER2(vm, ins).floating_point;
}
void in_binop_fp_compare_l(instruction_t ins, vm_t * vm){
	vm->registers[X].word= REGISTER1(vm, ins).floating_point<REGISTER2(vm, ins).floating_point;
}
void in_binop_int_to_float(instruction_t ins, vm_t * vm){
	REGISTER1(vm,ins).integer = REGISTER2(vm,ins).floating_point;	
}
void in_binop_float_to_int(instruction_t ins, vm_t * vm){
	REGISTER1(vm,ins).floating_point = REGISTER2(vm,ins).integer;
}
void in_push(instruction_t ins, vm_t * vm){
	WORD(vm->memory[vm->registers[SP].word])= REGISTER1(vm, ins);
	vm->registers[SP].word+= sizeof(uint32_t);
}
void in_pop(instruction_t ins, vm_t * vm){
	REGISTER1(vm, ins) = WORD(vm->memory[vm->registers[SP].word-1]);
	vm->registers[SP].word-= sizeof(uint32_t);
}
void in_halt(instruction_t ins, vm_t * vm){
	vm->flags.is_halted = true;
}
void in_syscall(instruction_t ins, vm_t * vm){
	uint32_t r = vm->registers[A0].word;	
	if(r == 0){
		printf("%d\n", vm->registers[A1].integer);
	} else if(r == 1){
		printf("%c\n", vm->registers[A1].bytes[0]);
	} else if(r == 2){
		printf("%.*s\n", vm->registers[A2].integer,(char*)&vm->memory[vm->registers[A1].word]);
	} else if(r == 3){
		fgets((char*)&vm->memory[vm->registers[A1].word], vm->registers[A2].integer, stdin);
	} else if(r == 4){
	}
}
void debug_vm(vm_t * vm){	
	for(int i =0; i<16; i++){
		printf("%s: %u, %d, %f\n", register_names[i],vm->registers[i].word, vm->registers[i].integer, vm->registers[i].floating_point);
	}
}
bool run_instruction(vm_t* vm){
	instruction_t ins = vm->instructions[vm->registers[IP].word];
	printf("%s: %s, %s\n", instruction_type_names[ins.type], register_names[ins.register1], register_names[ins.register2]);
	switch(ins.type){
		case nothing: 
			break;
		case load:
			in_load(ins, vm);
			break;
		case store:
			in_store(ins, vm);
			break;
		case load_absolute:
			in_load_abs(ins, vm);
			break;
		case store_absolute:
			in_store_abs(ins, vm);
			break;
		case store_pointer:
			in_store_pointer(ins, vm);
			break;
		case load_pointer:
			in_load_pointer(ins, vm);
			break;
		case mov:
			in_mov(ins,vm);
			break;
		case mov_immediate:
			in_mov_imm(ins,vm);
			break;
		case call:
			in_call(ins, vm);
			break;
		case ret:
			in_ret(ins, vm);
			break;
		case jmp:
			in_jmp(ins, vm);
			break;
		case jmp_register:
			in_jmp_register(ins, vm);
			break;
		case conditional_jmp:
			in_conditional_jmp(ins, vm);
			break;
		case conditional_jmp_register:
			in_conditional_jmp_register(ins, vm);
			break;
		case binop_add:
			in_binop_add(ins ,vm);
			break;
		case binop_subtract:
			in_binop_subtract(ins,vm);
			break;
		case binop_divide:
			in_binop_divide(ins ,vm);
			break;
		case binop_multiply:
			in_binop_multiply(ins ,vm);
			break;
		case binop_compare_g:
			in_binop_compare_g(ins ,vm);
			break;
		case binop_compare_e: 
			in_binop_compare_e(ins ,vm);
			break;
		case binop_compare_l:
			in_binop_compare_l(ins ,vm);
			break;
		case binop_unsigned_compare_g: 
			in_binop_unsigned_compare_g(ins ,vm);
			break;
		case binop_unsigned_compare_e: 
			in_binop_unsigned_compare_e(ins ,vm);
			break;
		case binop_unsigned_compare_l:
			in_binop_unsigned_compare_l(ins ,vm);
			break;
		case binop_fp_add:
			in_binop_fp_add(ins ,vm);
			break;
		case binop_fp_subtract:
			in_binop_fp_subtract(ins ,vm);
			break;
		case binop_fp_divide:
			in_binop_fp_divide(ins ,vm);
			break;
		case binop_fp_multiply:
			in_binop_fp_multiply(ins ,vm);
			break;
		case binop_fp_compare_g:
			in_binop_fp_compare_g(ins ,vm);
			break;
		case binop_fp_compare_e:
			in_binop_fp_compare_e(ins ,vm);
			break;
		case binop_fp_compare_l:
			in_binop_fp_compare_l(ins ,vm);
			break;
		case  int_to_float:
			in_binop_int_to_float(ins, vm);
			break;
		case float_to_int:
			in_binop_float_to_int(ins, vm);
			break;
		case push:
			in_push(ins, vm);
			break;
		case pop:
			in_pop(ins, vm);
			break;
		case halt:
			in_halt(ins, vm);
			break;
		case vm_syscall:
			in_syscall(ins, vm);
			break;
		default:
			printf("%u\n", ins.type);
			todo("pain");
	}
	vm->registers[IP].word++;
	return !vm->flags.is_halted;
}
const char * instruction_type_names[] = {
	"nothing", "load", "store", "load_absolute", 
	"store_absolute", "mov", "move_immediate","store_pointer","load_pointer", 
	"call", "ret","jmp","conditional_jmp",
	"jmp_register", "conditional_jmp_register", "add", "subtract", "divide", "multiply",
	"compare_g", "compare_e", "compare_l", "unsigned_compare_g", 
	"unsigned_compare_e", "unsigned_compare_l","fp_add", "fp_subtract", "fp_divide", "fp_multiply",
	"fp_compare_g", "fp_compare_e", "fp_compare_l","int_to_float","float_to_int", "push", "pop", "halt", "syscall"};
const char * register_names []=  {
	"IP", "BP", "SP", "R0", "R1", "A0", "A1", "A2", "A3", "A4", "A5", "A6", "A7","X", "Y", "Z"
};
