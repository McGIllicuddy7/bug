#pragma once
#include <stdlib.h>
#include <stdint.h>
#define MEMORY_SIZE 4096*4
#define INSTRUCTION_SIZE 4096*4
#define REGISTER_COUNT 16
#define IP 0
#define BP 1
#define SP 2
#define R0  3
#define R1 4
#define A0 5
#define A1 6
#define A2 7
#define A3 8
#define A4 9
#define A5 10
#define A6 11
#define A7 12
#define X 13
#define Y 14
#define Z 15

typedef enum:uint8_t{
	nothing = 0,	
	load, 
	store,
	load_absolute,
	store_absolute,
	mov,	
	mov_immediate,
	store_pointer, 
	load_pointer,
	call, 
	ret,
	jmp,
	conditional_jmp,
	jmp_register,
	conditional_jmp_register,
	binop_add,
	binop_subtract,
	binop_divide,
	binop_multiply, 
	binop_compare_g,
	binop_compare_e, 
	binop_compare_l,
	binop_unsigned_compare_g, 
	binop_unsigned_compare_e, 
	binop_unsigned_compare_l,
	binop_fp_add,
	binop_fp_subtract,
	binop_fp_divide,
	binop_fp_multiply, 
	binop_fp_compare_g,
	binop_fp_compare_e, 
	binop_fp_compare_l,
	int_to_float, 
	float_to_int,
	push, 
	pop,
	halt,
	vm_syscall,
	instruction_count,
}instrtype_t;
extern const char * instruction_type_names[];
extern const char * register_names[];
typedef union{
	uint32_t word;
	uint16_t shorts[2];
	uint8_t bytes[4];
	int32_t integer;
	float floating_point;
}memory_t; 
typedef struct {
	union{ 
		struct{
			int16_t offset;
			instrtype_t type;	
			uint8_t register1:4;
			uint8_t register2:4;
		
		};
		uint32_t data;
	};
}instruction_t;
typedef struct{
	uint8_t is_halted:1;
}flags_t;
typedef struct {
	unsigned char memory[MEMORY_SIZE];
	uint32_t memory_size;
	instruction_t instructions[INSTRUCTION_SIZE];
	uint32_t instructions_size;
	memory_t registers[REGISTER_COUNT];
	flags_t flags;
}vm_t;
bool run_instruction(vm_t * vm);
void debug_vm(vm_t*vm);
