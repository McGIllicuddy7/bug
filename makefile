make: main.c tokenizer.c parser.c
	gcc main.c tokenizer.c parser.c -lm -Wall
