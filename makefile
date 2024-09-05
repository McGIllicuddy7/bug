make: builtins.c output/test.s
	nasm -f macho64 output/test.s
	gcc output/test.o builtins.c -std=c2x -fsanitize=address