#include "bug.h"
typedef enum{
	TTEmpty, 	
	TTChar, 
	TTNum, 
	TTString,
	TTIdent,
}TTState;
TokenType calc_tt(char c){
	if(c == '('){
		return TokenOpenParen;
	}else if(c == ')'){
		return TokenCloseParen;
	}else if(c == '{'){
		return TokenOpenCurl;
	}else if(c == '}'){
		return TokenCloseCurl;
	}else if(c == '['){
		return TokenOpenBracket;
	}else if(c == ']'){
		return TokenCloseBracket;
	}else if(c == ';'){
		return TokenSemi;
	}else if(c == ':'){
		return TokenColon;
	}else if(c == '.'){
		return TokenDot;
	}else if(c == ','){
		return TokenComma;
	}
	else{
		return TokenOperator;
	}
}
bool is_whitespace(char c){

	return (c == ' ' || c  == '\n' || c == '\t' || c== '\r');
}
bool is_reserved(char c){
	return (c == '(' || c == ')' || c == '+' || c == '-' || c == '*' || c== '/' || c == '{' || c== '}' || c== ';' || c== '.' || c == '[' || c== ']'|| c== '.' || c == '<' || c == '=' || c== '>' || c == ':');
}
TokenResult next_token(TokenStream * strm){
	String s = new_string(strm->arena, "");
	Token out;
	TTState state = TTEmpty;
	bool was_slash = false;
	while(strm->index <strm->end){
		char c = strm->base[strm->index];
		strm->index++;
		if(c == '\n'){
			strm->line++;
		}
	restart:
		if(state == TTEmpty){
			if(is_whitespace(c)){
				continue;
			}else{
				if(c == '"'){
					state = TTString;
					str_v_append(s,c);
					continue;
				}else if(c == '\''){
					state = TTChar;
					str_v_append(s,c);
					continue;
				}else if(is_number(c)){
					state = TTNum;
					str_v_append(s,c);
					continue;
				}else if(is_reserved(c)){
					bool has_next = strm->index<strm->end;
					str_v_append(s,c);
					if(has_next){
						char n = strm->base[strm->index];
						if((c == '-' && n == '>') ||((c == '+' || c== '-' || c == '/' || c== '*' || c== '=' || c== '<' || c== '>') && n == '=')){
							strm->index++;
							str_v_append(s,n);	
						}	
					}
					out.tt = calc_tt(c);
					goto done;
				}else{
					state = TTIdent;
					goto restart;
				}	
			}
		}else{	
			if(state ==TTChar){
				if(was_slash){
					if(c == 't'){
						str_v_append(s, '\t');
					}else if(c == '"'){
						str_v_append(s, '"');
					}else if(c == 'n'){
						str_v_append(s, '\n');
					}else if(c == '0'){
						str_v_append(s,'\0');
					}
					was_slash = false;
				}
				else{
					if(c == '\\'){
						was_slash = true;
					}else{
						str_v_append(s, c);
						if(c == '\''){
						goto done;
						}
						was_slash = false;
					}
				}
			}else if(state == TTNum){
				if(is_number(c)){
					str_v_append(s,c);
				}else{
					strm->index--;
					goto done;
				}
			}else if(state == TTString){
				if(was_slash){
					if(c == 't'){
						str_v_append(s, '\t');
					}else if(c == '"'){
						str_v_append(s, '"');
					}else if(c == 'n'){
						str_v_append(s, '\n');
					}else if(c == '0'){
						str_v_append(s,'\0');
					}
					was_slash = false;
				}else{
					if(c == '\\'){
						was_slash = true;
					}else{
						str_v_append(s, c);
						if(c == '"'){
							goto done;
						}
						was_slash = false;
					}
				}
			}else if(state == TTIdent){
				if(is_whitespace(c) || is_reserved(c)){
					strm->index--;
					if(c == '\n'){
						strm->line--;
					}
					goto done;
				}else{
					str_v_append(s,c);
				}
			}
		}
	}	
	return Err(Token);
done:
	out.file = strm->file;
	out.line = strm->line;
	out.str = string_to_str(s);
	if(state == TTNum){
		out.tt = TokenInt;
	}else if(state == TTChar){
		out.tt = TokenChar;
	}else if(state == TTString){
		out.tt = TokenStr;
	}else if(state == TTIdent){
		out.tt = TokenIdent;
	}
	return Ok(Token, out);
}
const char * token_names[TokenTypeCount]= {"TokenNone",
	"TokenStr", 
	"TokenChar",
	"TokenInt", 
	"TokenFloat",
	"TokenOpenCurl",
	"TokenCloseCurl",
	"TokenOpenParen", 
	"TokenCloseParen",
	"TokenSemi",
	"TokenColon",
	"TokenOperator",
	"TokenIdent",
	"TokenComma", 
	"TokenDot", 
	"TokenOpenBracket",
	"TokenCloseBracket"};
#define STRF(s) (int)(s.length), s.items
void print_token(Token t){
	printf("{tt:%s, text:\"%.*s\", file:%.*s,line:%zu}\n",token_names[t.tt], STRF(t.str), STRF(t.file), t.line);
}

TokenStream create_token_stream(Arena * arena, Str str, Str file_name){
	TokenStream out;
	out.arena = arena;
	out.base = str.items;
	out.end = str.length;
	out.line = 1;
	out.file = file_name;
	out.index =0;
	return out;
}

TokenResult peek_token(TokenStream * strm){
	TokenStream st = *strm;
	TokenResult out = next_token(strm);
	*strm = st;
	return out;
}
