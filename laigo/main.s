.extern _interupt
.global _main
.extern _printf
_main:
    sub sp, sp, #16
    str fp, [sp, #16]
    str lr, [sp, #16]
    mov fp, sp
    sub sp, sp, 16
   adrp x0, _msg@PAGE
    add x0, x0, _msg@PAGEOFF
   adrp x0, _msg@PAGE
    add x0, x0, _msg@PAGEOFF
    mov x1, 4613937818241073152
    str x1, [fp,#-0]
    mov x1, fp
    sub x1, x1, 0
   ldr d0, [x1,#0]
    str d0, [sp, #-0]
   addv d0, d0, d0
    bl _printf
    mov sp, fp
    ldr fp, [sp, #8]
   ldr lr, [sp, #16]
    add sp, sp, #16
    ret
    mov sp, fp
    ldr fp, [sp, #8]
   ldr lr, [sp, #16]
    add sp, sp, #16
    ret
.data
_flt:
.byte 37,102,10,0
_msg:
.byte 104,101,121,32,116,111,97,115,116,32,105,32,108,111,118,101,32,121,111,117,32,58,37,102,10,0
