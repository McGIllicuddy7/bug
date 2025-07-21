#include "asm.h"
#include "../utils.h"
static size_t hashstr(Str s){
	return hash_bytes((unsigned char*)s.items, s.length);
}
enable_hash_type(Str,int);
enable_vec_type(instruction_t);
typedef struct {
	StrintHashTable *memory_table;
	StrintHashTable *lable_table;
	instruction_tVec instructions;
	size_t end;
}program_t;
uint8_t expect_register(Str s){
	if(str_equals(s, STR("ip"))){
		return IP;
	}
	else if(str_equals(s, STR("bp"))){
		return BP;
	}
	else if(str_equals(s, STR("sp"))){
		return SP;
	}
	else if(str_equals(s, STR("r0"))){
		return R0;
	}
	else if(str_equals(s, STR("r1"))){
		return R1;
	}
	else if(str_equals(s, STR("a0"))){
		return A0;
	}
	else if(str_equals(s, STR("a1"))){
		return A1;
	}	
	else if(str_equals(s, STR("a2"))){
		return A2;
	}	
	else if(str_equals(s, STR("a3"))){
		return A3;
	}
	else if(str_equals(s, STR("a4"))){
		return A4;
	}
	else if(str_equals(s, STR("a5"))){
		return A5;
	}
	else if(str_equals(s, STR("a6"))){
		return A6;
	}
	else if(str_equals(s, STR("a7"))){
		return A7;
	}
	else if(str_equals(s, STR("x"))){
		return X;
	}
	else if(str_equals(s, STR("y"))){
		return Y;
	}
	else if(str_equals(s, STR("z"))){
		return Z;
	}
}
int16_t expect_offset(Arena * arena,Str s){
	return atoi(str_to_c_string(arena,s));
}
uint32_t expect_address(Arena * arena,Str s,program_t * prog){
	if(StrintHashTable_contains(prog->memory_table, s)){
		return *StrintHashTable_find(prog->memory_table,s);
	} else{
		atoi(str_to_c_string(arena, s));
	}
}
uint32_t expect_ip(Arena * arena,Str s,program_t * prog){
	if(StrintHashTable_contains(prog->lable_table, s)){
		return *StrintHashTable_find(prog->lable_table,s);
	} else{
		atoi(str_to_c_string(arena, s));
	}
}
#define EXPECT(v, count) if(v.length+1<count) printf("ERROR expected arguments");return;
void compile_line(Arena*arena,Str line, program_t * prog){
	StrVec strs = str_split_by_delim(arena,line, STR(" "));
	if(strs.length<1 ) return;
	Str s = strs.items[0];
	instruction_t is;
	is.data =0;
	if(str_equals(s, STR("noop"))){
		v_append(prog->instructions,is);
	} else if(str_equals(s, STR("load"))){	
		EXPECT(strs, 2);
		is.type = load;
		is.register1 = expect_register(strs.items[1]);
		is.offset = expect_offset(arena,strs.items[2]);
		v_append(prog->instructions,is);
	}
	else if(str_equals(s, STR("store"))){
		EXPECT(strs,2);
		is.type = store;
		is.register1 = expect_register(strs.items[1]);
		is.offset = expect_offset(arena,strs.items[2]);
		v_append(prog->instructions,is);
	}
	else if(str_equals(s, STR("load_abs"))){
		EXPECT(strs,2);
		is.type = load_absolute;
		is.register1 = expect_register(strs.items[1]);
		v_append(prog->instructions,is);
		instruction_t s;
		s.data = expect_address(arena, strs.items[2], prog);
		v_append(prog->instructions,s);
	}
	else if(str_equals(s, STR("store_abs"))){
		EXPECT(strs,2);
		is.type = store_absolute;
		is.register1 = expect_register(strs.items[1]);
		v_append(prog->instructions,is);
		instruction_t s;
		s.data = expect_address(arena, strs.items[2], prog);
		v_append(prog->instructions,s);
	}
	else if(str_equals(s, STR("mov"))){
		EXPECT(strs,2);
		is.type = mov;
		is.register1 = expect_register(strs.items[1]);
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions,is);
	}
	else if(str_equals(s, STR("mov_imm"))){
		EXPECT(strs,2);
		is.type=  mov_immediate;
		is.register1 = expect_register(strs.items[1]);
		v_append(prog->instructions,is);
		instruction_t s;
		s.data = expect_address(arena, strs.items[2], prog);
		v_append(prog->instructions,s);
	}
	else if(str_equals(s, STR("sp"))){
		EXPECT(strs,2);
		is.type = store_pointer;
		is.register1 = expect_register(strs.items[1]);
		is.register2 = expect_register(strs.items[2]);
		is.offset =0;
		if(strs.length>3){
			is.offset = expect_offset(arena, strs.items[3]);
		}
		v_append(prog->instructions,is);
	}
	else if(str_equals(s, STR("lp"))){
		EXPECT(strs,2);
		is.type = load_pointer;
		is.register1 = expect_register(strs.items[1]);
		is.register2 = expect_register(strs.items[2]);
		is.offset =0;
		if(strs.length>3){
			is.offset = expect_offset(arena, strs.items[3]);
		}
		v_append(prog->instructions,is);
	}
	else if(str_equals(s, STR("call"))){
		EXPECT(strs,1);
		is.type = call;
		v_append(prog->instructions,is);
		instruction_t s;
		s.data = expect_ip(arena, strs.items[1],prog);
		v_append(prog->instructions,s);	
	}
	else if(str_equals(s, STR("ret"))){
		is.type = ret;
		v_append(prog->instructions,is);
	}
	else if(str_equals(s, STR("jmp"))){
		EXPECT(strs,1);
		is.type = jmp;
		v_append(prog->instructions,is);
		instruction_t s;
		s.data = expect_ip(arena, strs.items[1],prog);
		v_append(prog->instructions,s);	
	}
	else if(str_equals(s, STR("cjmp"))){
		EXPECT(strs,2);
		is.type = conditional_jmp;
		is.register1 = expect_register(strs.items[1]);
		v_append(prog->instructions,is);
		instruction_t s;
		s.data = expect_ip(arena, strs.items[2],prog);
		v_append(prog->instructions,s);	
	}
	else if(str_equals(s, STR("jmp_reg"))){
		EXPECT(strs,1);
		is.type = jmp_register;
		is.register1 = expect_register(strs.items[1]);	
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("cjmp_reg"))){
		EXPECT(strs,2);
		is.type = conditional_jmp_register;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR("+"))){
		EXPECT(strs,2);
		is.type = binop_add;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR("-"))){
		EXPECT(strs,2);
		is.type = binop_subtract;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR("*"))){
		EXPECT(strs,2);
		is.type = binop_multiply;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR("/"))){
		EXPECT(strs,2);
		is.type = binop_divide;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR(">"))){
		EXPECT(strs,2);
		is.type = binop_compare_g;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR("=="))){
		EXPECT(strs,2);
		is.type = binop_compare_e;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR("<"))){
		EXPECT(strs,2);
		is.type = binop_compare_l;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR(">u"))){
		EXPECT(strs,2);
		is.type = binop_unsigned_compare_g;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR("==u"))){
		EXPECT(strs,2);
		is.type = binop_unsigned_compare_e;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR("<u"))){
		EXPECT(strs,2);
		is.type = binop_unsigned_compare_l;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}

	else if(str_equals(s, STR("+f"))){
		EXPECT(strs,2);
		is.type = binop_fp_add;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);	
	}
	else if(str_equals(s, STR("-f"))){
		EXPECT(strs,2);
		is.type = binop_fp_subtract;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("*f"))){
		EXPECT(strs,2);
		is.type = binop_fp_multiply;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("/f"))){
		EXPECT(strs,2);
		is.type = binop_fp_divide;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR(">f"))){
		EXPECT(strs,2);
		is.type = binop_fp_compare_g;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("==f"))){
		EXPECT(strs,2);
		is.type = binop_fp_compare_e;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("<f"))){
		EXPECT(strs,2);
		is.type = binop_fp_compare_l;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("itof"))){
		EXPECT(strs,2);
		is.type = int_to_float;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("ftoi"))){
		EXPECT(strs,2);
		is.type = float_to_int;
		is.register1 = expect_register(strs.items[1]);	
		is.register2 = expect_register(strs.items[2]);
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("push"))){
		EXPECT(strs,1);
		is.type = push;
		is.register1 = expect_register(strs.items[1]);	
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("pop"))){
		EXPECT(strs,1);
		is.type = pop;
		is.register1 = expect_register(strs.items[1]);	
		v_append(prog->instructions, is);

	}
	else if(str_equals(s, STR("halt"))){
		is.type = halt;	
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("sys"))){
		is.type = vm_syscall;	
		v_append(prog->instructions, is);
	}
	else if(str_equals(s, STR("label"))){
		EXPECT(strs,1);
		StrintHashTable_insert(prog->lable_table, strs.items[1], prog->instructions.length);
	}
	else if(str_equals(s, STR("dw"))){
		size_t size = expect_offset(arena,strs.items[2]);
		if(size%4 !=0){
			size += 4-size%4;
		}
		size_t base = prog->end-size;
		prog->end-= size;

		StrintHashTable_insert(prog->memory_table, strs.items[1], base);
	}
	else if(str_equals(s, STR(";"))){
		return;	
	}
}
vm_t* compile_string(const char * string){
	Arena * arena = arena_create();
	StrVec lines= str_split_by_delim_no_delims(arena, STR(string), STR("\n"));
	program_t pg;
	pg.instructions = make(arena, instruction_t);
	pg.lable_table = StrintHashTable_create(4096, hashstr, str_equals, (void(*)(Str*))no_op_void, (void(*)(int*))no_op_void);
	pg.memory_table = StrintHashTable_create(4096, hashstr, str_equals, (void(*)(Str*))no_op_void, (void (*)(int *))no_op_void);
	pg.end = MEMORY_SIZE-1;
	for(size_t i =0; i<lines.length; i++){
		compile_line(arena, lines.items[i], &pg);
	}
	pg.instructions.length =0;
	for(size_t i =0; i<lines.length; i++){
		compile_line(arena, lines.items[i], &pg);
	}
	vm_t *out =(vm_t*)calloc(1, sizeof(vm_t));
	memcpy(out->instructions, pg.instructions.items, pg.instructions.length*sizeof(instruction_t));
	StrintHashTable_unmake(pg.memory_table);
	StrintHashTable_unmake(pg.lable_table);
	arena_destroy(arena);
	return out;
}
