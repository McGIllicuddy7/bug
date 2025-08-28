.extern _interupt
.global _main
_main:
   sub sp, sp ,#16
   str fp, [sp, #-8]
   str lr ,[sp, #-16]
   mov fp, sp
   add fp, fp, #16
    mov x1, 2
    mov x2, 2
   add x1, x2, x0
    mov x3, x0
    mov x0, 0
   mov sp, fp
 l dr lr, [sp, #8]
 l dr fp,[sp]
