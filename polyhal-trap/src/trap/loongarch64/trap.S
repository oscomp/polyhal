    .equ    LA_CSR_CRMD,          0x0
    .equ    LA_CSR_PRMD,          0x1
    .equ    LA_PLV_USER,          0x3

    .section .text
    .global     trap_vector_base
    .type       trap_vector_base, @function
    .p2align    12
// Check whether it was from user privilege.
trap_vector_base:
    csrwr   $sp, KSAVE_USP
    csrrd   $sp, LA_CSR_PRMD
    andi    $sp, $sp, LA_PLV_USER
    bnez    $sp, {user_vec} 

    csrrd   $sp, KSAVE_USP
    addi.d  $sp, $sp, -{trapframe_size} // allocate space

    // save the registers.

    SAVE_REGS

    move    $a0, $sp
    bl      {trap_handler}

    // Load registers from sp, include new sp
    LOAD_REGS
    ertn
.size trap_vector_base, . - trap_vector_base

save_float_regs:
    .set    n, 0
    .rept   32
    fst.d   $f\n, $a0, n * 8
    .set    n, n + 1
    .endr
.size save_float_regs, . - save_float_regs
