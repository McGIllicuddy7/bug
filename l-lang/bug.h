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
	o_ad, //add
	o_sb, //subtract
	o_ml, //multiply
	o_dv, //divide
	o_as, //assign
	o_gt,//goto
	o_cgt,//conditional_goto
	o_dec,//declare
	o_type,//type operator
	o_num,//number operator 
	o_flt,//float operator
	o_str,//string operator
	o_idnt,//indentifer operator
	o_fld,//field access
	o_call,//call
	o_function,//function_operator	
	o_auto_dec,//auto_declare
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
typedef enum {
	StatementDeclare,
	StatementExpr,
	StatementIf, 
	StatementWhile,
	StatementFor,
}StatementType;
typedef struct Statement{
	StatementType st;
	Expr *expr;
	struct Statement * children;
	size_t count;
}Statement;
enable_result(Statement);
enable_vec_type(Statement);
typedef enum {
	t_int, 
	t_double, 
	t_string, 
	t_char, 
	t_void,
	t_struct,
	t_function,
}TypeT;
struct Type;
typedef struct {
	Str name;
	struct Type * type;
}Field;
typedef struct Type{	
	TypeT v_type;
	union {
		struct{
			struct Type * ret;
			Field * args;
			size_t arg_count;
		}func;
		struct {
			Field * fields;
			size_t field_count;
		}strct;
	};
	
}Type;
enable_vec_type(Type);
typedef struct{
	Str name;
	Type return_type;
	Field * args;
	size_t Arg_count;
}Function;
enable_vec_type(Function);

TokenResult next_token(TokenStream * strm);
TokenResult peek_token(TokenStream * strm);
bool token_equals(Token t, const char * ptr);
void print_token(Token t);
TokenStream create_token_stream(Arena * arena, Str str, Str file_name);
ExprResult parse_expression(Arena * arena,Token * tokens, size_t count);
void print_expr(Expr s);

