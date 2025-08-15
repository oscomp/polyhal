    .equ    LA_CSR_CRMD,          0x0
    .equ    LA_CSR_PRMD,          0x1
    .equ    LA_CSR_EUEN,          0x2
    .equ    LA_CSR_DMW0,          0x180
    .equ    LA_CSR_DMW1,          0x181
    .equ    LA_IOCSR_MAIL_BUF0,   0x1020
    .equ    LA_IOCSR_MAIL_BUF1,   0x1028
    .equ    LA_IOCSR_MAIL_BUF2,   0x1030
    .equ    LA_IOCSR_MAIL_BUF3,   0x1038

    .equ    CSR_DMW1_MAT,         0x10
    .equ    CSR_DMW1_PLV0,        0x1

    .equ    MAT_SUC,              0             // Strongly-ordered UnCached
    .equ    MAT_CC,               1             // Coherent Cached
    .equ    MAT_WUC,              2             // Weakly-ordered UnCached

    .equ    CSR_CRMD_PG,          1 << 4
    .equ    CSR_CRMD_DATF,        (MAT_CC << 5)             // MAT of Fetch
    .equ    CSR_CRMD_DATM,        (MAT_CC << 7)             // MAT of Memory Load/Store
    .macro INIT_DMW
        ori         $t0, $zero, CSR_DMW1_PLV0
        lu52i.d     $t0, $t0, -2048     # UC, PLV0, 0x8000 xxxx xxxx xxxx
        csrwr       $t0, LA_CSR_DMW0

        ori         $t0, $zero, CSR_DMW1_PLV0 | CSR_DMW1_MAT
        lu52i.d     $t0, $t0, -1792     # CA, PLV0, 0x9000 xxxx xxxx xxxx
        csrwr       $t0, LA_CSR_DMW1
    .endm
    .macro INIT_CSR
        CSRWRI      LA_CSR_CRMD, $t0, CSR_CRMD_DATF | CSR_CRMD_DATM | CSR_CRMD_PG
        CSRWRI      LA_CSR_PRMD, $t0, 0x00                            # PLV=0, PIE=0, PWE=0
        CSRWRI      LA_CSR_EUEN, $t0, 0x00                            # FPE=0, SXE=0, ASXE=0, BTE=0
    .endm

    .macro CSRWRI  CSR, REG, VALUE
        li.w    \REG, \VALUE
        csrwr   \REG, \CSR
    .endm

    .section .text.entry
    .global _start
    .type   _start, @function
_start:
    INIT_DMW
    INIT_CSR
    bl          clear_bss

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
    INIT_CSR

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
