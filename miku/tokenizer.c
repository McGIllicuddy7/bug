#include "tokenizer.h"
#include <stdbool.h>
#include <assert.h>
#include <stdio.h>
const char * TOKEN_STRINGS[] = {"+", "-", "*", "/", "=", "(", "{", ")", "}", ",", ";", ":", "variable", "end"};

Tokenizer tokenizer_init(const char * str, const char * file){
	return (Tokenizer){str, str, file, 1};
}
static Token parse_string_literal(Tokenizer* tokenizer){	
	size_t count;
	Token out;
	bool hit;
	bool last_was_backslash;
	count = 0;
	last_was_backslash = false;
	hit = false;
	out.start = tokenizer->current;
	out.line = tokenizer->line;
	out.file = tokenizer->file;	
	tokenizer->current++;
	while(*tokenizer->current){
		printf("current char:%c\n", *tokenizer->current);
		switch (*tokenizer->current){
			case '\\': last_was_backslash= true;
			break;
			case '"':{
				if(!last_was_backslash) hit = true;
				last_was_backslash = false;
				break;
			}
			case '\n': tokenizer->line++;		
			default:
				last_was_backslash = false;
		}
		tokenizer->current++;	
		if(hit){out.len = count+1; tokenizer->current+=1; return out;}	
		if (!hit){
			tokenizer->current++;
			count++;
		} else{
			break;
		}
	}
	return out;
}
static void consume_white_space(Tokenizer * tokenizer){
	while(*tokenizer->current){
		switch (*tokenizer->current){
			case '\n':
			tokenizer->line++;
			case ' ':
			case '\t':
			tokenizer->current++;	
			break;
			default:
			return;
		}
	}
}
Token next_token(Tokenizer* tokenizer){
	const char * start;
	size_t count;
	consume_white_space(tokenizer);
	start = tokenizer->current;
	count = 0;			
	#ifdef C23
	__attribute__ ((fallthrough))
	#endif	
	switch (*(tokenizer->current)){
		case '+':
		case '-':
		case '/':
		case '*':
		case '=':
		case '(':
		case ')':
		case ';':
		case ',':
		case ':':
		case '{':
		case '}':
		tokenizer->current++;
		count = 1;
		break;
		case '\0':
		return (Token){0,0,0,0};
		count = 0;
		case '"':
			printf("string lit\n");
			return parse_string_literal(tokenizer);
		break;			
		tokenizer->current++;
		tokenizer->line ++;
		default:	
		while(*(tokenizer->current)){
			bool hit = false;
			#ifdef C23
			__attribute__ ((fallthrough))
			#endif	
			switch (*(tokenizer->current)){
				case '+':
				case '-':
				case '/':
				case '*':
				case '=':
				case '(':
				case ')':
				case '{':
				case '}':
				case ';':
				case ',':	
				case ':':
				hit  = true;
				break;
				case '\n': tokenizer->line++;
				case ' ':	
				hit = true;
				tokenizer->current++;
				break;
				case '"':
				printf("string lit\n");
				return parse_string_literal(tokenizer);
				break;
				default:
					tokenizer->current++;
					count++;
			}
			if (hit) break;
		}
		break;
		
	}
	Token out;
	out.file = tokenizer->file;
	out.line = tokenizer->line;
	out.start = start;
	out.len = count;	
	return out;
}
TokenType get_token_type(Token t){
	char c;
	if(t.len == 0){
		return TokenEnd;
	}	
	if (t.len == 1){
		c = t.start[0];
		switch(c){
			case '+':return TokenPlus;
			case '-':return TokenMinus;
			case '*':return TokenMul;
			case '/':return TokenDiv;
			case '=':return TokenAssign;
			case '(':return TokenOpenParen;
			case ')':return TokenCloseParen;
			case ';':return TokenSemiColon;
			case ',': return TokenComma;
			case ':': return TokenColon;
			case '{': return TokenOpenCurly;
			case '}':return TokenCloseCurly;
		}
	}	
	return TokenVariable;
}

void print_token(Token t){
	printf("<%.*s>\n",(int)t.len,t.start);	
}
TokenBuff tokenizer_collect(Tokenizer * tokenizer, Arena * arena){
	size_t count =0;
	Tokenizer clone = *tokenizer;
	while(true){
		Token t = next_token(&clone);
		count++;
		if(get_token_type(t)== TokenEnd){
			break;
		}
		
	}
	Token* out = arena_alloc(arena,sizeof(Token)*count);
	for(size_t i = 0; i<count; i++){
		out[i] = next_token(tokenizer);
	}
	TokenBuff to_return;
	to_return.items = out;
	to_return.count = count;
	return to_return;
}
