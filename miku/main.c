#define CTILS_IMPLEMENTATION
#include "utils.h"
#include "tokenizer.h"
#include "expr.h"
void print_tokens(const char * buff){
	Tokenizer tokenizer;
	tokenizer = tokenizer_init(buff, "stdin");
	while(true){
		Token tok = next_token(&tokenizer);
		if(get_token_type(tok) == TokenEnd){
			break;
		}else{
			print_token(tok);
		}
	}
}
void print_expression(Arena * arena,const char * buff){	
	Tokenizer tokenizer = tokenizer_init(buff, "stdin");
	TokenBuff tb = tokenizer_collect(&tokenizer,arena);
	Expr * exp = parse_expression(arena,tb.items, tb.count);
	print_expr(exp);
}
void print_statement_call(Arena * arena,const char * buff){	
	Tokenizer tokenizer = tokenizer_init(buff, "stdin");
	TokenBuff tb = tokenizer_collect(&tokenizer,arena);	
	Statement sp= parse_statement(arena, tb.items, tb.count);
	print_statement(sp);
}
void repl_loop(){
	Arena * arena = arena_create();
	char buff[4096];
	while(true){
		memset(buff,0, sizeof(buff));
		fgets(buff, sizeof(buff)-1, stdin);
		if (!strcmp(buff, "exit\n")){
			break;	
		}
		print_tokens(buff);
		print_statement_call(arena, buff);
		arena_reset(arena);
	}
	arena_destroy(arena);

}
void compile(const char * name){
	Arena * arena = arena_create();
	String s = read_file_to_string(arena, name);
	Tokenizer tok = tokenizer_init(s.items, name);
	TokenBuff tb = tokenizer_collect(&tok, arena);
	TreeProgram *prog = parse_tree_program(arena,tb.items, tb.count);
	print_program(prog);
	arena_destroy(arena);
}
int main(int argc, const char ** argv){
	compile("main.bug");
}
