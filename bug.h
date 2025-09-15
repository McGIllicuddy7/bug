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
typedef enum{
	o_add, 
	o_sub, 
	o_div, 
	o_mul, 
	o_asn, 
}OpType;
typedef enum {
	v_immf,
	v_immi,
	v_stc,
	v_nm,
	v_take_ref, 
	v_deref,
	v_index,
}VarType;
typedef struct Operand{
	VarType type;
	int i;
	float f;
	int idx;
	Str nm;
	struct Operand * child;
}Operand;

typedef struct {
	OpType type;
	Operand * arguments;
	size_t arg_count;
	long output;
}Operation;
enable_vec_type(Operation);
typedef struct{
	OperationVec operations;
}Expr;
enable_result(Expr);
typedef enum {
	If, 
	While,
}StatementType;
TokenResult next_token(TokenStream * strm);
TokenResult peek_token(TokenStream * strm);
void print_token(Token t);
TokenStream create_token_stream(Arena * arena, Str str, Str file_name);
ExprResult parse_expression(Arena * arena,Token * tokens, size_t count);


