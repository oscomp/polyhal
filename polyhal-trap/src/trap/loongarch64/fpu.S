    .macro FPU_SAVE dst, tmp
	movcf2gr	\tmp, $fcc0
	move	    \dst, \tmp
	movcf2gr	\tmp, $fcc1
	bstrins.d	\dst, \tmp, 15, 8
	movcf2gr	\tmp, $fcc2
	bstrins.d	\dst, \tmp, 23, 16
	movcf2gr	\tmp, $fcc3
	bstrins.d	\dst, \tmp, 31, 24
	movcf2gr	\tmp, $fcc4
	bstrins.d	\dst, \tmp, 39, 32
	movcf2gr	\tmp, $fcc5
	bstrins.d	\dst, \tmp, 47, 40
	movcf2gr	\tmp, $fcc6
	bstrins.d	\dst, \tmp, 55, 48
	movcf2gr	\tmp, $fcc7
	bstrins.d	\dst, \tmp, 63, 56
	.endm

	.macro FPU_RESTORE src, tmp
	bstrpick.d	 \tmp, \src, 7, 0
	movgr2cf	$fcc0, \tmp
	bstrpick.d	 \tmp, \src, 15, 8
	movgr2cf	$fcc1, \tmp
	bstrpick.d	 \tmp, \src, 23, 16
	movgr2cf	$fcc2, \tmp
	bstrpick.d	 \tmp, \src, 31, 24
	movgr2cf	$fcc3, \tmp
	bstrpick.d	 \tmp, \src, 39, 32
	movgr2cf	$fcc4, \tmp
	bstrpick.d	 \tmp, \src, 47, 40
	movgr2cf	$fcc5, \tmp
	bstrpick.d	 \tmp, \src, 55, 48
	movgr2cf	$fcc6, \tmp
	bstrpick.d	 \tmp, \src, 63, 56
	movgr2cf	$fcc7, \tmp
	.endm
