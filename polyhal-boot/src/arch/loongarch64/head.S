    .equ    LA_CSR_CRMD,          0x0
    .equ    LA_CSR_PRMD,          0x1
    .equ    LA_CSR_EUEN,          0x2
    .equ    LA_CSR_DMW0,          0x180
    .equ    LA_CSR_DMW1,          0x181
    .equ    LA_IOCSR_MAIL_BUF0,   0x1020
    .equ    LA_IOCSR_MAIL_BUF1,   0x1028
    .equ    LA_IOCSR_MAIL_BUF2,   0x1030
    .equ    LA_IOCSR_MAIL_BUF3,   0x1038

    .macro INIT_DMW
        ori         $t0, $zero, 0x1     # CSR_DMW1_PLV0
        lu52i.d     $t0, $t0, -2048     # UC, PLV0, 0x8000 xxxx xxxx xxxx
        csrwr       $t0, LA_CSR_DMW0

        ori         $t0, $zero, 0x11    # CSR_DMW1_MAT | CSR_DMW1_PLV0
        lu52i.d     $t0, $t0, -1792     # CA, PLV0, 0x9000 xxxx xxxx xxxx
        csrwr       $t0, LA_CSR_DMW1
    .endm

    .macro CSRWRI  CSR, REG, VALUE
        li.w    \REG, \VALUE
        csrwr   \REG, \CSR
    .endm

    .section .text
    .global _start
    .type   _start, @function
_start:
    bl          clear_bss
    INIT_DMW
    # Enable PG
    CSRWRI      LA_CSR_CRMD, $t0, 0xb0      # PLV=0, IE=0, PG=1
    CSRWRI      LA_CSR_PRMD, $t0, 0x00      # PLV=0, PIE=0, PWE=0
    CSRWRI      LA_CSR_EUEN, $t0, 0x00      # FPE=0, SXE=0, ASXE=0, BTE=0

    la.global   $sp, bstack_top
    csrrd       $a0, 0x20           # cpuid
    la.global   $t0, {entry}
    jirl        $zero, $t0, 0
    .size _start, . - _start

    # secondary_start function
    .global _secondary_start
    .type _secondary_start, @function
_secondary_start:
    INIT_DMW

    li.w        $t0, LA_IOCSR_MAIL_BUF1    # Load Stack Pointer From Message Buffer
    iocsrrd.d   $sp, $t0

    csrrd       $a0, 0x20                  # cpuid
    la.global   $t0, {secondary_entry}

    jirl        $zero, $t0, 0
    .size _secondary_start, . - _secondary_start

    # Clear BSS section, write a byte in the L1
    .type clear_bss, @function
clear_bss:
    la.global    $t0, _sbss
    la.global    $t1, _ebss
.L1:
    st.b         $zero, $t0, 0
    addi.d       $t0,   $t0, 1
    blt          $t0,   $t1, .L1
    ret
    .size clear_bss, . - clear_bss
