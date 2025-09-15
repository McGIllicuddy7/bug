#define CTILS_IMPLEMENTATION
#include "utils.h"
#include "bug.h"
int main(int argc, const char ** argv){
	Arena * local = arena_create();
	Str s = string_to_str(read_file_to_string(local, "tokenizer.c"));
	TokenStream ts = create_token_stream(local, s, STR("tokenizer.c"));
	for(TokenResult t = next_token(&ts); t.data.ok; t = next_token(&ts)){
		Token st = t.value;	
		print_token(st);
	}
}
