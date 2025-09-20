#include "bug.h"
#define unit ((Unit){})
bool token_equals(Token t, const char * ptr){
	Str s = STR(ptr);
	return str_equals(t.str, s);
}
typedef enum {
	Er,
	OpenParen,
	CloseParen,
	Add, 
	Sub, 
	Mul, 
	Div,
	Call, 
	Dot,
	Assign,
}Op;
enable_vec_type(Op);
enable_result(Op);
typedef struct {
	Token t;
}Var;
enable_vec_type(Var);
typedef struct {
	OprVec oprs;
	OpVec ops; 
	VarVec vars; 
}Eval;
static int op_prior(Op o){
	if(o == Dot){
		return 0;
	}
	else if(o == Mul){
		return 1;
	}else if(o == Div){
		return 1;
	}else if(o == Sub){
		return 2;
	}else if(o == Add){
		return 2;
	}else if(o == Assign){
		return 3;
	}
}
OpResult pop_op(Eval * ev){
	if(ev->oprs.length>0){
		Op o = ev->ops.items[ev->ops.length-1];
		ev->ops.length--;
		return Ok(Op, o);
	}
	return Err(Op);
}
void push_op(Eval * ev, Op o){
	v_append(ev->ops, o);
}
UnitResult insert_op(Arena * arena,Eval * ev, Token t){
	Op o = Er;
	if(token_equals(t, "=")){
		o = Assign;
	}else if(token_equals(t, "+")){
		o = Add;
	}else if(token_equals(t, "-")){
		o = Sub;
	}else if(token_equals(t, "*")){
		o = Mul;
	}else if(token_equals(t, "/")){
		o = Div;
	}else{
		return Err(Unit);
	}
	OpResult op;
	if(ev->ops.length == 0){
		v_append(ev->ops, o);
		return Ok(Unit, unit);
	}
	OpVec remainder = make(arena, Op);
	int p0 = op_prior(o);
	while((op = pop_op(ev)).data.ok){
		Op ol = op.value;
		int p1 = op_prior(ol);
		if(ol== OpenParen || (p0>p1)){
			break;
		}
	}
	return Ok(Unit, unit);
}

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
			v_append(eval.vars,tmp);
		}else{
			return Err(Expr);
		}
	}
	out.ops = eval.oprs;
	return Ok(Expr, out);
}
