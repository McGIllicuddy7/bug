#include "bug.h"
#define unit ((Unit){})
bool token_equals(Token t, const char * ptr){
	Str s = STR(ptr);
	return str_equals(t.str, s);
}

static bool is_numbers(Token t){
	for(int i =0; i<t.str.length; i++){
		if(!is_number(t.str.items[i]) || t.str.items[i] == '.'){
			return false;
		}
	}
	return true;
}

static bool is_dec(Token t){
	bool hit_dot = false;
	for(int i =0; i<t.str.length; i++){
		if(t.str.items[i] == '.'){
			if(hit_dot){
				return false;
			}else{
				hit_dot = true;
			}
		}else if(!is_number(t.str.items[i])){
			return false;
		}
	}
	return true;
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
	Colon,
	ColonEquals,
	Assign,
}Op;
enable_vec_type(Op);
enable_result(Op);

typedef struct {
	Token t;
	int sv;
}Var;
enable_vec_type(Var);

typedef struct {
	OprVec oprs;
	OpVec ops; 
	VarVec vars; 
	int sp;
}Eval;

static int op_prior(Op o){
	if(o == Mul){
		return 2;
	}else if(o == Div){
		return 2;
	}else if(o == Sub){
		return 1;
	}else if(o == Add){
		return 1;
	}else if(o == Assign){
		return 0;
	}else if(o == Dot){
		return 3;
	}else if(o == Colon){
		return 4;
	}else if(o == ColonEquals){
		return 4;
	}else if(o == OpenParen){
		-1;
	}else{
		todo();
	}
	return -1;
}

long get_next_outside_of_expr(Token * tokens,size_t start, size_t count, TokenType t){
	long paren_count = 0;
	for(int i =start; i<count; i++){
		if(paren_count == 0){
			if(tokens[i].tt == t){
				printf("found\n");
				return i;
			}
		}
		if(tokens[i].tt == TokenOpenParen){
			paren_count ++;
		}else if(tokens[i].tt == TokenCloseParen){
			paren_count--;
		}
	}
	return -1;
}
long get_scope_end(Token * tokens, size_t start, size_t count){
	long e = 0;
	int i =start;
	do{
		if(tokens[i].tt == TokenOpenBracket){
			e++;
		}else if(tokens[i].tt == TokenCloseBracket){
			e--;
		}
		i++;
		if(i>=count){
			return -1;
		}
	}while(e >0);
	return e;
}
void write_var(Eval * ev, Var v){	
	if(v.sv != -1){
		return;
	}
	Opr o;	
	if(v.t.tt == TokenInt){
		o.t = o_num;
		char buff[100];
		snprintf(buff, 99, "%.*s", (int)v.t.str.length, v.t.str.items);
		o.v = atol(buff);
		
	}else if(v.t.tt == TokenFloat){
		o.t = o_num;
		char buff[100];
		snprintf(buff, 99, "%.*s", (int)v.t.str.length, v.t.str.items);
		o.d = atof(buff);
	}
	else if(v.t.tt == TokenStr){
		o.t = o_str;
		o.s = v.t.str;
	}else if(v.t.tt == TokenIdent){
		o.t = o_idnt;
		o.s = v.t.str;
	}else{
		todo();
		return;
	}
	v_append(ev->oprs, o);
}

UnitResult eval_op(Op o,Eval * ev){
	Var v0  = ev->vars.items[ev->vars.length-1];
	ev->vars.length--;
	Var v1  = ev->vars.items[ev->vars.length-1];
	ev->vars.length--;
	write_var(ev, v0);
	write_var(ev, v1);
	Opr op;
	if(o == Add){
		op.t = o_ad;
	}else if(o == Sub){
		op.t = o_sb;
	}else if(o == Div){
		op.t = o_dv;
	}else if(o == Mul){
		op.t = o_ml;
	}else if(o == Assign){
		op.t = o_as;
	}else if(o == Dot){
		op.t = o_fld;
	}else if(o == Colon){
		op.t = o_dec;
	}else if(o == ColonEquals){
		op.t = o_auto_dec;
	}else{
		printf("error not a valid op\n");
		todo();
		exit(1);
	}
	v_append(ev->oprs, op);
	return Ok(Unit, unit);
}

ExprResult parse_expression(Arena * arena, Token * tokens, size_t count){
	Arena * local = arena_create();
	Expr out;
	Eval ev;
	ev.ops = make(local, Op);
	ev.oprs = make(arena, Opr);
	ev.vars = make(local, Var);
	bool last_was_v = false;	
	for(int i =0; i<count; i++){	
		if(tokens[i].tt == TokenInt || tokens[i].tt == TokenFloat || tokens[i].tt == TokenStr || tokens[i].tt == TokenIdent){
			Var v;
			v.t = tokens[i];
			v.sv = -1;
			v_append(ev.vars, v);
			last_was_v = true;
		}else if(tokens[i].tt == TokenOpenParen){
			if(last_was_v){		
     				i++;
				Var v = ev.vars.items[ev.vars.length-1];
				ev.vars.length--;
				long end = get_next_outside_of_expr(tokens, i, count, TokenCloseParen);
				if(end == -1){
					return Err(Expr);
				}
				int arg_count =0;
				for(; i<end; i++){
					long e = get_next_outside_of_expr(tokens, i, count, TokenComma);
					if(e == -1 || e>=end){
						e = end;	
					}
					ExprResult er=parse_expression(arena, tokens+i, e-i);
					if(!er.data.ok){
						return Err(Expr);
					}
					Expr ep = er.value;
					arg_count++;
					for(int j=0; j<ep.ops.length; j++){
						v_append(ev.oprs, ep.ops.items[j]);
					}
					i = e;
				}
				Opr op;
				op.t = o_idnt;
				op.s = v.t.str;
				v_append(ev.oprs, op);
				i= end+1;
				last_was_v = true;
				op.v = arg_count;
				op.t = o_call;
				v_append(ev.oprs, op);
		
			}else{
				last_was_v = false;
				v_append(ev.ops, OpenParen);
			}
		}else if(tokens[i].tt == TokenCloseParen){
			while(ev.ops.items[ev.ops.length-1] != OpenParen){
				Op o = ev.ops.items[ev.ops.length-1];
				ev.ops.length--;
				eval_op(o, &ev);
			}
			ev.ops.items--;
			last_was_v = true;
		}else if(tokens[i].tt == TokenDot || tokens[i].tt == TokenOperator || tokens[i].tt == TokenColon){
			Op o;
			if(tokens[i].tt== TokenDot){
				o = Dot;
			}else if(token_equals(tokens[i], "+")){
				o = Add;
			}
			else if(token_equals(tokens[i], "-")){	
				o = Sub;
			}
			else if(token_equals(tokens[i], "*")){	
				o = Mul;
			}
			else if(token_equals(tokens[i], "/")){	
				o = Div;
			}
			else if(token_equals(tokens[i], "=")){	
				o = Assign;
			}else if(token_equals(tokens[i], ":")){
				o = Colon;
			}else if(token_equals(tokens[i], ":=")){
				o = ColonEquals;
			}else{
				return Err(Expr);
			}
			while(ev.ops.length>0){
				if(op_prior(ev.ops.items[ev.ops.length-1])<op_prior(o)){	
						break;
				}
				ev.ops.length--;
				if(o == OpenParen){
					break;
				}
				eval_op(o, &ev);
			}
			v_append(ev.ops, o);
			last_was_v = false;
		}else{
			print_token(tokens[i]);
			return Err(Expr);
		}
	}
	if(ev.ops.length == 0){
		if(ev.vars.length == 1){
			write_var(&ev, ev.vars.items[0]);
			ev.vars.length--;
		}
	}
	while(ev.ops.length>0){
		Op o = ev.ops.items[ev.ops.length-1];	
		if(o == OpenParen|| o == CloseParen){
			ev.ops.length--;
			continue;
		}
		ev.ops.length--;	
		eval_op(o, &ev);
	}
	out.ops = ev.oprs;
	return Ok(Expr, out);
}
long get_statement_end(Token *tokens, size_t start, size_t count){
	Token t = tokens[start];
	if(token_equals(t, "{")){
		return get_scope_end(tokens, start,count);
	}else if(token_equals(t, "if") || token_equals(t, "while")){
		long e= get_next_outside_of_expr(tokens, 1, count, TokenCloseParen);
		if(e == -1){
			return -1;
		}
		return get_statement_end(tokens, e+1, count);
	}else{
		return get_next_outside_of_expr(tokens, start, count, TokenSemi);
	}
	return -1;
}
StatementResult parse_statement(Arena * arena, Token * tokens, size_t count, FunctionVec * funcs,TypeVec * types){
	Statement out = {0};
	if(count == 0){
		out.st = StatementExpr;
		return Ok(Statement, out);
	}
	else if(token_equals(tokens[0], "if")){
		long e= get_next_outside_of_expr(tokens, 1, count, TokenCloseParen);
		if(e == -1){
			return Err(Statement);
		}
		Expr s;
		Try(Expr, Statement,p, parse_expression(arena, tokens+1, e-1), {s = p;});
		Statement stmnt;
		Try(Statement, Statement, p, parse_statement(arena, tokens+e+1, count-e-1, funcs, types),stmnt = p;);	
		todo();
	}else if(token_equals(tokens[0], "while")){
		long e= get_next_outside_of_expr(tokens, 1, count, TokenCloseParen);
		if(e == -1){
			return Err(Statement);
		}
		Expr s;
		Try(Expr, Statement,p, parse_expression(arena, tokens+1, e-1), {s = p;});
		Statement stmnt;
		Try(Statement, Statement, p, parse_statement(arena, tokens+e+1, count-e-1, funcs, types),{stmnt = p;});
		todo();
	}else if(token_equals(tokens[0], "{")){
		long e = get_scope_end(tokens, 1, count);
		long i =2;
		while(i<e){
			Token t= tokens[i];
			long end = get_statement_end(tokens, i, count);
			i = end;

		}
		
	}else{
		long end = get_next_outside_of_expr(tokens, 0, count, TokenSemi);
		if(end == -1){
			return Err(Statement);
		}
		else{
			ExprResult e = parse_expression(arena,tokens, count);
			if(!e.data.ok){
				return Err(Statement);
			}
			Expr * exp = arena_alloc(arena, sizeof(Expr));
			*exp = e.value;
			out.expr = exp;
			out.st = StatementExpr;
			return Ok(Statement, out);
		}
	}
	return Err(Statement);
}

void print_expr(Expr s){
	const char * op_names[] = {"+", "-", "*", "/", "=", "unknown what this is", "num", "float", "str", "ident", "field", "call", 0};
	for(int i =0; i<len(s.ops); i++){
		Opr o = s.ops.items[i];
		if(o.t < o_num){
			printf("exp:%s\n",op_names[o.t]);
		}else if(o.t == o_num){
			printf("exp:%ld\n",o.v);
		}else if(o.t == o_flt){
			printf("exp:%f\n", o.d);
		}else if(o.t == o_str || o.t == o_idnt){
			printf("exp ident:%.*s\n", (int)(o.s.length), o.s.items);
		}else if(o.t == o_call){
			printf("call:%ld\n", o.v);
		}else if(o.t == o_fld){
			printf("field_access\n");
		}else{
			todo();
		}
	}
}

