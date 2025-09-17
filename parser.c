#include "bug.h"
bool token_equals(Token t, const char * ptr){
	Str s = STR(ptr);
	return str_equals(t.str, s);
}
typedef enum {
	OpenParen,
	CloseParen,
	Add, 
	Sub, 
	Mul, 
	Div,
	Call, 
	Assign,
}Op;
enable_vec_type(Op);
typedef struct {
	Token t;
}Var;
enable_vec_type(Var);
typedef struct {
	OprVec oprs;
	OpVec ops; 
	VarVec vars; 
}Eval;

ExprResult parse_expression(Arena * arena,Token * tokens, size_t count){
	Expr out;
	Eval eval;
	eval.oprs = make(arena, Opr);
	eval.ops = make(arena, Op);
	eval.vars =make(arena, Var);
	for(size_t i =0; i<count; i++){
		Token t = tokens[i];
		if(t.tt == TokenOperator || t.tt == TokenDot){
			todo();	
		}else if(t.tt ==TokenOpenParen){
			todo();
		}else if(t.tt == TokenCloseParen){
			todo();
		}else if(t.tt == TokenStr || t.tt == TokenIdent || t.tt == TokenFloat || t.tt == TokenInt){
			Var tmp;
			tmp.t = t;
			v_append(vars,tmp);
		}else{
			return Err(Expr);
		}
	}
	return Ok(Expr, out);
}
