#pragma once 
#include <stdlib.h>
#include "utils.h"
typedef struct{
	const char * start;
	size_t len;
	const char * file;
	size_t line;
}Token;
typedef enum{
	TokenPlus, 
	TokenMinus, 
	TokenMul, 
	TokenDiv, 
	TokenAssign,
	TokenOpenParen, 
	TokenOpenCurly,
	TokenCloseParen, 
	TokenCloseCurly,
	TokenComma, 
	TokenSemiColon, 
	TokenColon,
	TokenVariable,
	TokenEnd
} TokenType;
extern const char * TOKEN_STRINGS[];
typedef struct {
	const char * current; 
	const char * base;
	const char * file;
	size_t line;
}Tokenizer;
typedef struct {
	Token* items;
	size_t count;
}TokenBuff;
Tokenizer tokenizer_init(const char * str, const char * file_name);
Token next_token(Tokenizer* tokenizer);
TokenBuff tokenizer_collect(Tokenizer*tokenizer, Arena * arena);
TokenType get_token_type(Token t);
void print_token(Token token);
bool token_equals(Token token, const char * str);
