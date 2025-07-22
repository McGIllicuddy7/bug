halt
label add
	+ a0 a1
	mov r0 a0
	ret
label main
	mov_imm a0 2
	mov_imm a1 40
	call add
	mov a1 r0
	mov_imm a0 0
	sys
	halt
