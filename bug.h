#pragma once 
#include "utils.h"
typedef enum{
	TokenStr, 
	TokenInt, 
	TokenFloat,
	TokenOpenCurl,
	TokenCloseCurl,
	TokenOpenParen, 
	TokenCloseParen,
	TokenIdent,
}TokenType;
typedef struct {
	Str str;
	Str file;
	TokenType tt;
}Token;
enable_result(Token);
typedef struct {
	const char * base;
	size_t index;
	size_t end;
}TokenStream;
TokenResult next_token(TokenStream * strm);
