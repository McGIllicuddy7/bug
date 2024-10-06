Bug is supposed to be a statically typed, compiled, garbage collected programming language, it is my first attempt at making something like this and it is EXTREMLY BROKEN in ways that are nondeterministic due to my not understanding how rust hashmaps work well enough. Do not use it. If you want to do anything with it: 
struct (struct name){
  (name of field):(type of field)
  (name of field):(type of field)
  .
  .
  .
}; is how you define types.
fn (name of function)(arg:type arg:type arg:type...) ->(return type){stuff in function}, is how you define a function. functions can be overloaded based on argument types
let name = thing; or let name:type = thing; is how you create a new variable. 
you call a function by either function_name(arg1, arg2 ,arg3...) or arg1.function_name(arg2, arg3...), or you can define an operator of a pair of types as a function e.g fn +(a:int b:int)->int{return a+n;} and use the operator as you would for a primitive type. 
Speaking of primitive types you have :
char: a character
bool: a boolean 
float: a 64 bit float, can be printed using put_str and put_str_ln
int: a 64 bit signed integer, can be printed using put_str and put_str_ln
string: a string, can be added to other strings and printed using put_str and put_str_ln, I have not gotten non alphanumeric characters working yet. 
[]type: a slice of that type, containing an array and a length, can be made using make(type, length)
^type: a pointer to a t, can be dereferenced using ^(thing to deref) or made using either &(a value of type) or new (type). 
void: only for function return values, a function that returns nothing. 

you can import other files using the import directive, import (filename). this will import all functions and types that have the pub prefix in the file recursively. so a pub struct foo{} would get imported but struct baz{} would not 
in all other respects this language is essentially garbage collected c. for loops use let, while loops exist. do while loops do not, goto does not, if and else statements work as expected and there are no bitwise operations. finally you can use put_str and put_str_ln to print a single string, float, or int, by writing put_str(value), there are builtin casts that are functions named the new type that turn ints into floats and vice versa and to_string turns an int or float into a string. 

NOTE: currently bug will only compiler the file called test.bug in the directory, this will change someday but for now just put the root at test.bug
an example bug program 

pub fn main ()->int{
  put_str_ln("hello world");
  return 0;
}
