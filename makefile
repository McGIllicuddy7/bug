make: builtins.c output/test.s
	nasm -f macho64 output/test.s
	clang output/gc_functions.c output/test.o builtins.c -std=c2x 
