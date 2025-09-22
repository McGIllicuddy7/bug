#define CTILS_IMPLEMENTATION
#include "utils.h"
#include "bug.h"
enable_vec_type(Token);
void pfile(){
	Arena * local = arena_create();
	Str s = string_to_str(read_file_to_string(local, "tokenizer.c"));
	TokenStream ts = create_token_stream(local, s, STR("tokenizer.c"));
	for(TokenResult t = next_token(&ts); t.data.ok; t = next_token(&ts)){
		Token st = t.value;	
		print_token(st);
	}

}
int main(int argc, const char ** argv){
	Arena * local = arena_create();
	while(true){
		char buff[4096] = {0};
		fgets(buff, 4095, stdin);
		if(str_equals(STR(buff), STR("exit\n"))){
			break;
		}
		TokenStream ts = create_token_stream(local, STR(buff), STR("stdin"));
		TokenVec tokens = make(local, Token);
		for(TokenResult t = next_token(&ts); t.data.ok; t = next_token(&ts)){	
			v_append(tokens, t.value);
		}
		ExprResult exp= parse_expression(local, tokens.items, tokens.length);
		if(!exp.data.ok){
			printf("%s\n", exp.data.msg);
			const char ** x = exp.data.stack_trace;
			while(*x){
				printf("%s\n", *x);
				x++;
			}
			return -1;
		}
		Expr e = exp.value;
		print_expr(e);
		arena_reset(local);
	}
}

