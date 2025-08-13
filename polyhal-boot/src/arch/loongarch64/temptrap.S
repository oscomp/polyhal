    .macro STD  offset
        st.d  $r\offset, $sp, \offset * 8
    .endm

    .macro LDD  offset
        ld.d  $r\offset, $sp, \offset * 8
    .endm

    .section .text
    .global _temptrap_entry
    .type _temptrap_entry, @function
    .p2align 12
_temptrap_entry:
    addi.d  $sp, $sp, -32 * 8
    STD      1
    STD      2
    STD      4
    STD      5
    STD      6
    STD      7
    STD      8
    STD      9
    STD     10
    STD     11
    STD     12
    STD     13
    STD     14
    STD     15
    STD     16
    STD     17
    STD     18
    STD     19
    STD     20
    STD     21
    STD     22
    STD     23
    STD     24
    STD     25
    STD     26
    STD     27
    STD     28
    STD     29
    STD     30
    STD     31

    move    $a0, $sp
    bl      handle_exception

    LDD     31
    LDD     30
    LDD     29
    LDD     28
    LDD     27
    LDD     26
    LDD     25
    LDD     24
    LDD     23
    LDD     22
    LDD     21
    LDD     20
    LDD     19
    LDD     18
    LDD     17
    LDD     16
    LDD     15
    LDD     14
    LDD     13
    LDD     12
    LDD     11
    LDD     10
    LDD      9
    LDD      8
    LDD      7
    LDD      6
    LDD      5
    LDD      4
    LDD      2
    LDD      1
    addi.d  $sp, $sp, 32 * 8
    ertn
    .size _temptrap_entry, . - _temptrap_entry
