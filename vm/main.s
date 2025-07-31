label fib
	;mov a1 a0
	;mov_imm a0 0
	;sys
	;mov a0 a1
	mov_imm y 2
	< a0 y
	cjmp x end1
	mov_imm y 1
	- a0 y
	call fib	
	mov a2 r0
	- a0 y
	call fib
	+ r0 a2
	ret
	label end1
	mov_imm r0 1
	ret
label main
	mov_imm y 0
	push y
label loop
	mov_imm x 1
	+ y x
	mov a0 y
	call fib
	mov a1 r0
	mov x y
	mov_imm a0 0
	sys
	mov_imm x 100
	<  y x
	cjmp x loop
	halt
