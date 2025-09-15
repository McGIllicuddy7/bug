#pragma once 
#include "utils.h"
typedef enum{
	TokenNone = 0,
	TokenStr, 
	TokenChar,
	TokenInt, 
	TokenFloat,
	TokenOpenCurl,
	TokenCloseCurl,
	TokenOpenParen, 
	TokenCloseParen,
	TokenSemi,
	TokenColon,
	TokenOperator,
	TokenIdent,
	TokenComma, 
	TokenDot, 
	TokenOpenBracket,
	TokenCloseBracket,
	TokenTypeCount,
}TokenType;
typedef struct {
	Str str;
	Str file;
	size_t line;
	TokenType tt;
}Token;
enable_result(Token);
typedef struct {
	Str file;
	size_t index;
	size_t end;
	size_t line;
	const char * base;
	Arena * arena;
}TokenStream;
typedef enum {
	NodeIndent, 
	NodeFuncCall,
	NodeEq,
	NodeBinop,
	NodeList,
}NodeType;

TokenResult next_token(TokenStream * strm);
TokenResult peek_token(TokenStream * strm);
void print_token(Token t);
TokenStream create_token_stream(Arena * arena, Str str, Str file_name);

