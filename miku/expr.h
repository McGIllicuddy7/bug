#pragma once 
#include "tokenizer.h"
#include "utils.h"
typedef enum {
	ExprBin,
	ExprUn,
	ExprVar,
	ExprParen,
	ExprCall,
	ExprTypeCount,
}ExprType;
extern char * expr_strings[ExprTypeCount];
typedef struct Expr {
	struct Expr * left;
	struct Expr * right;
	struct Expr * parent;
	Token value;
	ExprType type;
} Expr;

typedef enum {
	StatementIf, 
	StatementWhile, 
	StatementFor,
	StatementScope,
	StatementBasic,
} StatementType;
struct Scope;
typedef struct Statement{
	StatementType statement_type;
	Expr* expr;
	Expr * init;
	Expr * iter;	
	struct TreeScope * scope;	
	struct TreeScope * else_scope;
}Statement;
enable_vec_type(Statement)
typedef struct TreeScope{
	Statement * statements;
	size_t count;
} TreeScope;
typedef struct {
	Expr * header;
	Statement statement;
}TreeFn;
typedef struct {
	Token name;
	StatementVec fields;
}TreePlex;
enable_vec_type(TreeFn);
enable_vec_type(TreePlex);
typedef struct {
	StatementVec global_statements;
	TreeFnVec functions;
	TreePlexVec plexes;
}TreeProgram;
Expr *parse_expression(Arena * arena,Token * tokens, size_t count);
Statement parse_statement(Arena * arena, Token* tokens, size_t count);
void print_expr(Expr * expr);
void print_statement(Statement s);
void print_program(TreeProgram * prog);
void print_tree_fn(TreeFn func);
void print_tree_plex(TreePlex plx);
TreeScope parse_scope(Arena * arena,Token * tokens,size_t count);
void parse_global(Arena * arena, TreeProgram * prog, Token* tokens, size_t count);
size_t get_global_size(Token*tokens,size_t count);
TreeProgram * parse_tree_program(Arena * arena,Token * tokens, size_t count);
