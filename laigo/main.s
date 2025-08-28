.intel_syntax noprefix
.extern interupt
.global main
main:
    push rbp
    mov rbp, rsp
    sub rsp, 8
    sub rsp,16
    mov rsi, 2
    mov rdx, 8
    push rdx
    imul rdx, rsi
    mov rsi, rdx
 pop rdx
    mov rdi, 0
    mov rsi, [rbp-0]
    call interupt
    mov rsi, [rbp-0]
   push rax
   cmp rsi, 2
   setg al
    movzx rdi,al
   pop rax
    cmp rdi,0
     je l1
    jmp l2
l1:
    mov rdi, 0
    mov rsi, 3
    call interupt
    jmp end
l2:
    mov rdi, 0
    mov rsi, 100
    call interupt
end:
mov rsp, rbp
    pop rbp
    ret
    mov rsp, rbp
    pop rbp
    ret