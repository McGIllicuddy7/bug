make: builtins.c output/test.s
	nasm -f macho64 output/test.s
	clang output/test.o builtins.c output/gc_functions.c -std=c2x 
