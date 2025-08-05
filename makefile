make: main.c prelude.c
	gcc main.c prelude.c -std=c23 -g3 -fsanitize=address
