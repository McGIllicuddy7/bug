make: builtins.c output/test.s
	nasm -f elf64 output/test.s
	clang output/test.o builtins.c -std=c2x 
