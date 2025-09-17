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
	o_ad, 
	o_sb, 
	o_ml, 
	o_dv, 
	o_as, 
	o_gt,
	o_num,
	o_flt,
	o_fld,
	o_call,
}OpType;
typedef struct {
	OpType t;
	union{
		Str s;
		long v;
		double d;
	};
}Opr;
enable_vec_type(Opr);
typedef struct {
	OprVec ops;
}Expr;
enable_vec_type(Expr);
enable_result(Expr);
TokenResult next_token(TokenStream * strm);
TokenResult peek_token(TokenStream * strm);
bool token_equals(Token t, const char * ptr);
void print_token(Token t);
TokenStream create_token_stream(Arena * arena, Str str, Str file_name);
ExprResult parse_expression(Arena * arena,Token * tokens, size_t count);


