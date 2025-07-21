#include "expr.h"
char * expr_strings[ExprTypeCount] = {"ExprBin", "ExprUn", "ExprVar", "ExprParen", 0};
size_t get_paren_end(Token * start, size_t count){
	size_t paren_count =0;
	size_t out =0;
	while(true){
		
		if(out >= count){
		for(int i =0; i<count; ++i)
				printf("   %.*s\n", (int)start[i].len, start[i].start);

			todo();
		}
		if(get_token_type(start[out])== TokenOpenParen){
			paren_count+=1;
		} else if (get_token_type(start[out]) == TokenCloseParen){
			paren_count -=1;
			if(paren_count ==0){
				return out;
			}
		}
		out+=1;
	}
}
size_t get_curly_end(Token* start, size_t count){
	size_t paren_count =0;
	size_t out =0;
	while(true){	
		if(out >= count){
	
		for(int i =0; i<count; ++i)
				printf("   %.*s\n", (int)start[i].len, start[i].start);
		todo("failed to get curly count");
		}
		if(get_token_type(start[out])== TokenOpenCurly){
			paren_count+=1;
		} else if (get_token_type(start[out]) == TokenCloseCurly){
			paren_count -=1;
			if(paren_count ==0){
				return out;
			}
		}
		out+=1;
	}
}
size_t get_next_of_type(Token*start, size_t count,TokenType needle){
	size_t out = 0;
	while(true){
		if(out>=count){
			printf("failed to find token:%s\nlist was:\n",TOKEN_STRINGS[needle]);
			for(int i =0; i<count; ++i)
				printf("   %.*s\n", (int)start[i].len, start[i].start);
			todo();
		}
		if(get_token_type(start[out]) == needle){
			return out;
		}
		out+= 1;
	}
}

static int get_priority(Expr *exp){
	if (exp->type == ExprVar){
		return 10;	
	}
	if(exp->type == ExprBin){
		char c = *(exp->value.start);
		switch(c){
		case '+':
			return 6;
		case '-':
			return 6;
		case '/':
			return 8;
		case '*':
			return 8;
		case ':':
			return 9;
		case '=':
			return 1;
		default:
			return 0;
		}
	}
	if(exp->type == ExprParen){
		return 10;
	}
	if(exp->type == ExprUn){
		return 10;
	}
	return 0;
}
Expr * bubble_up(Expr * prev, Expr *to_add){
	assert(to_add);
	assert(prev != to_add);
	if(!prev){	
		return to_add;
	}
	int prior = get_priority(to_add);
	int prev_prior= get_priority(prev);
	if(prev->type == ExprUn){
		if(!prev->right){
			to_add->parent = prev;
			prev->right = to_add;
			return prev;
		}
	}
	if(prev_prior<= prior){
		prev->right = to_add;
		to_add->parent= prev;
		return to_add;
	}	
	while(prev->parent){
		prev = prev->parent;
		if(get_priority(prev)<= prior){
			to_add->parent = prev;
			if(prev->right){
				to_add->left = prev->right;
			}
			prev->right = to_add;
			
			return to_add;
		}
	}
	to_add->left = prev;
	prev->parent = to_add;
	return to_add;
}

Expr* parse_expression(Arena * arena,Token * tokens, size_t count){
	Expr * out = 0;		
	ExprType prev_type = ExprBin;
	Expr * prev = 0;
	size_t idx =0;	
	while(true){
		print_token(tokens[idx]);

		if(idx>= count){
			break;
		}
		Token tok = tokens[idx];
		idx++;
		TokenType tt = get_token_type(tok);
		if (tt== TokenEnd){
			break;
		}
		if(tt == TokenOpenParen){
			size_t end = get_paren_end(&tokens[idx-1],count);	
			Expr *exp = parse_expression(arena, &tokens[idx], end-1);
			Expr * ep = (Expr*)arena_alloc(arena, sizeof(Expr));	
			ep->type = ExprParen;
			ep->parent =0;	
			ep->left =0;
			ep->right = exp;
			prev = bubble_up(prev, ep);
			idx += end;
			continue;
		} else if (tt == TokenCloseParen){
			todo("handle parens");
		} else if (tt == TokenComma){
			todo("handle commas");
		}
		Expr * exp = arena_alloc(arena, sizeof(Expr));
		memset(exp, 0, sizeof(Expr));
		exp->value = tok;	
		switch(*(tok.start)){
			case '+':	
			exp->type = ExprBin;
			break;
			case '*':
			exp->type = ExprBin;
			break;
			case '/':
			exp->type = ExprBin;
			break;
			case ':':
			exp->type = ExprBin;
			break;
			case '=':
			exp->type = ExprBin;
			break;
			case '-':
			if(prev_type != ExprVar){
				exp->type = ExprUn;	
			} else{
				exp->type = ExprBin;
			}
			break;
			default:
			exp->type = ExprVar;
			break;
		}
		prev_type = exp->type;	
		prev = bubble_up(prev, exp);
	}
	out = prev;
	if(!out){
		return out;
	}
	while(out->parent){	
		out = out->parent;
	}
	return out;
}
static void print_expr_internal(Expr * expr, int depth){
	for(int i =0; i<depth; i++){
		putc(' ', stdout);
	}
	puts("{");
	for(int i =0; i<depth+1; i++){
		putc(' ', stdout);
	}
	if(expr->type ==ExprParen){
		puts("()");
	}
	else{
		for(size_t i =0; i<expr->value.len; i++){
			putc(expr->value.start[i],stdout);
		}
	}
	putc('\n', stdout);
	if(expr->left){
		print_expr_internal(expr->left, depth+1);
	}
	else{	
		for(int i =0; i<depth+1; i++){
			putc(' ',stdout);
		}	
		puts("left ptr:0");
	}

	if(expr->right){
		print_expr_internal(expr->right, depth+1);
	}
	else{
		for(int i =0; i<depth+1; i++){
			putc(' ',stdout);
		}	
		puts("right ptr:0");
	}
	for(int i =0; i<depth+1; i++){
		putc(' ', stdout);
	}	
	printf("}\n");
}
void print_expr(Expr * expr){

	print_expr_internal(expr,0);
}
bool token_equals(Token token, const char * str){
	if(strlen(str)!= token.len){
		return 0;
	}else{
		for(size_t i =0;i<token.len; i++){
			if(token.start[i] != str[i]){
				return 0;
			}
		}
	}
	return 1;
}
size_t get_statement_size(Token* tokens, size_t count){
	Token initial = tokens[0];		
//	size_t idx = 0;
	if(token_equals(initial, "{")){
		size_t end =get_curly_end(tokens,count);
		return end+1;	
	}else if(token_equals(initial,"if")){
		size_t end = get_paren_end(tokens+1,count-1);	
		size_t scope_sz = get_statement_size(tokens+end+1, count-end-1);
		if(token_equals(tokens[end+scope_sz+1], "else")){
			size_t else_sz = get_statement_size(tokens+end+scope_sz+1,count-end-scope_sz-1);	
			return end+scope_sz+else_sz+2;	
		}
		return end+scope_sz+1;
	}else if(token_equals(initial, "while")){
		size_t end = get_paren_end(tokens+1, count-1);
		size_t scope_sz = get_statement_size(tokens+end+1, count-end-1);
		return end+scope_sz+1;
	}else {
		size_t end = get_next_of_type(tokens, count, TokenSemiColon);
		return end+1;
	}
}
Statement parse_statement(Arena * arena, Token* tokens, size_t count){
	Token initial = tokens[0];		
	printf("initial<%.*s>\n",(int)tokens[0].len,  tokens[0].start);
	if(token_equals(initial, "{")){
		size_t end =get_curly_end(tokens,count);
		TreeScope s= parse_scope(arena, tokens, end);
		Statement out;
		out.statement_type =StatementScope;
		out.scope = (TreeScope*)arena_alloc(arena,sizeof(TreeScope));
		*out.scope =s;
		return out;
	}else if(token_equals(initial,"if")){
		size_t end = get_paren_end(tokens+1,count-1)+1;	
		Expr * expr = parse_expression(arena, tokens+1, end);
		size_t scope_sz = get_statement_size(tokens+end+1, count-end-1)+1;
		Statement st= parse_statement(arena, tokens+end+1,scope_sz);
		Statement out;
		out.statement_type = StatementIf;
		TreeScope * scope = (TreeScope*)arena_alloc(arena, sizeof(TreeScope));
		scope->count =1;
		scope->statements = (Statement*)arena_alloc(arena,sizeof(Statement));
		*scope->statements = st;
		out.scope =scope;
		out.expr = expr;
		printf("%.*s\n", (int)tokens[end+scope_sz+1].len, tokens[end+scope_sz+1].start);
		if(token_equals(tokens[end+scope_sz+1], "else")){
			size_t else_sz = get_statement_size(tokens+end+scope_sz+2,count-end-scope_sz-1)+1;	
			Statement el = parse_statement(arena,tokens+end+scope_sz+2, else_sz);
			out.else_scope = (TreeScope*)arena_alloc(arena, sizeof(TreeScope));
			out.else_scope->count =1;
			out.else_scope->statements =(Statement*) arena_alloc(arena,sizeof(Statement));
			*(out.else_scope->statements) = el;
		}
		return out;
	}else if(token_equals(initial, "while")){
		Statement out;
		out.statement_type = StatementWhile;
		size_t end = get_paren_end(tokens+1, count)+1;
		out.expr = parse_expression(arena, tokens+1, end+1);
		size_t scope_sz = get_statement_size(tokens+end+1, count-end-1)+1;
		out.scope =(TreeScope*)arena_alloc(arena,sizeof(TreeScope));
		out.scope->statements =(Statement*) arena_alloc(arena, sizeof(Statement));
		out.scope->count =1;
		Statement lp = parse_statement(arena,tokens+end+1, scope_sz);
		*out.scope->statements = lp;
		return out;

	}else {
		size_t end = get_next_of_type(tokens, count, TokenSemiColon);
		Statement out;
		out.statement_type = StatementBasic;
		Expr* expr = parse_expression(arena, tokens, end);
		out.expr = expr;
		return out;
	}
	return (Statement){};
}

TreeScope parse_scope(Arena * arena,Token * tokens, size_t count){
	size_t idx =1;
	StatementVec statements = make(arena, Statement);
	while(idx<count){
		printf("%.*s\n", (int)(*(tokens+idx)).len, tokens[idx].start);
		size_t sc = get_statement_size(tokens+idx, count);
		Statement s =parse_statement(arena, tokens+idx, sc+1);
		v_append(statements,s);
		idx += sc;
	}	
	TreeScope out;
	out.count = statements.length;
	out.statements = statements.items;
	return out;
}
static inline void left_pad(size_t count){
	for(size_t i =0; i<count; i++){
		printf(" ");
	}
}
static void print_statement_internal(Statement s, size_t depth){
	left_pad(depth);
	printf("Statement{\n");
	left_pad(depth+1);
	const char * statement_table[] = {"if", "while", "for", "scope","basic"};
	printf("%s\n", statement_table[s.statement_type]);
	switch(s.statement_type){
		case StatementIf:
			print_expr_internal(s.expr, depth+1);
			print_statement_internal(s.scope->statements[0], depth+1);
			if(s.else_scope){
				print_statement_internal(s.else_scope->statements[0], depth+1);
			}
			break;
		case StatementWhile:	
			print_expr_internal(s.expr, depth+1);
			print_statement_internal(s.scope->statements[0], depth+1);	
			break;
		case StatementScope:
			for(int i =0; i<s.scope->count; i++){
				print_statement_internal(s.scope->statements[i], depth+1);
			}	
			break;
		case StatementBasic:
			print_expr_internal(s.expr, depth+1);	
			break;
		default:
			todo();
	}
	left_pad(depth);
	printf("}\n");
}
void print_statement(Statement s){
	print_statement_internal(s,0);

}
size_t get_function_size(Token* tokens, size_t count){
	size_t base = get_paren_end(tokens, count);
	size_t statement = get_statement_size(tokens+base, count-base);
	return statement+base;
}
size_t get_global_size(Token*tokens,size_t count){
	if(token_equals(*tokens, "(")){
		size_t end = get_paren_end(tokens,count);
		size_t st_end = get_statement_size(tokens+end+1, count-end-1);
		return end+st_end;
	}
	else if(token_equals(*tokens, "plex")){
		Token name = tokens[1];
		size_t scope = get_curly_end(tokens+2, count-2);
		return scope+2;
		}
	else {
		size_t end =get_statement_size(tokens,count);
		return end+1;
	}
}
void parse_global(Arena * arena, TreeProgram * prog,Token* tokens, size_t count){
	if(token_equals(*tokens, "(")){
		size_t end = get_paren_end(tokens,count);
		Expr * exp = parse_expression(arena, tokens,end+1);
		size_t sz = get_statement_size(tokens+end+1, count);
		Statement st = parse_statement(arena, tokens+end+1, sz+1);
		TreeFn func;
		func.statement = st;
		func.header = exp;
		v_append(prog->functions,func);
	}
	else if(token_equals(*tokens, "plex")){
		Token name = tokens[1];
		size_t scope = get_curly_end(tokens+2, count-2);
		size_t idx =3;
		StatementVec v = make(arena, Statement);
		while(idx<scope-1){
			size_t end = get_statement_size(tokens+idx,scope-1-idx);
			Statement st = parse_statement(arena, tokens+idx, scope-1-idx);
			v_append(v, st);
		}
		TreePlex plx;
		plx.name = name;
		plx.fields = v;
	}
	else {
		size_t end =get_statement_size(tokens,count)+1;
		Statement s= parse_statement(arena, tokens,end);
		v_append(prog->global_statements,s);
	}
}
void print_tree_fn_internal(TreeFn func, size_t count){
	left_pad(count);
	printf("{");
	print_expr_internal(func.header, count+1);
	print_statement_internal(func.statement, count+1);
	left_pad(count);
	printf("{\n");
}
void print_tree_fn(TreeFn func){
	print_tree_fn_internal(func, 0);
}
void print_tree_plex_internal(TreePlex plx, size_t count){
	left_pad(count);
	printf("%.*s{", (int)plx.name.len, plx.name.start);
	for(size_t i =0; i<plx.fields.length; i++){
		print_statement_internal(plx.fields.items[i],count+1);
	}
	left_pad(count);
	printf("}\n");
}
void print_tree_plex(TreePlex plx){
	print_tree_plex_internal(plx,0);
}
void print_program(TreeProgram * prog){
	printf("{");
	for(size_t i =0; i<prog->plexes.length; ++i){
		print_tree_plex_internal(prog->plexes.items[i],1);	
	}
	for(size_t i =0; i<prog->global_statements.length; i++){
		print_statement_internal(prog->global_statements.items[i],1);
	}
	for(size_t i =0; i<len(prog->functions); i++){
		print_tree_fn_internal(prog->functions.items[i], 1);
	}
}
TreeProgram * parse_tree_program(Arena * arena,Token * tokens, size_t count){
	size_t idx =0; 
	TreeProgram * prog = (TreeProgram*)arena_alloc(arena, sizeof(TreeProgram));
	prog->functions = make(arena, TreeFn);
	prog->plexes = make(arena, TreePlex);
	prog->global_statements = make(arena, Statement);
	while(idx<count-1){
		size_t len = get_global_size(tokens+idx, count-idx);
		parse_global(arena, prog, tokens+idx, len);
		idx += len+1;
	}
	return prog;
}
