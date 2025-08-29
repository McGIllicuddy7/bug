.extern _interupt
.global _main
.extern _hello_world
_main:
    sub sp, sp, #16
    str fp, [sp, #16]
    str lr, [sp, #16]
    mov fp, sp
    sub sp, sp, 0
   adrp x0, _hello_world@PAGE
    add x0, x0, _hello_world@PAGEOFF
    blr x0
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
