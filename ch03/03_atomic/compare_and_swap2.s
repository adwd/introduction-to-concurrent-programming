	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0	sdk_version 12, 1
	.globl	_compare_and_swap               ; -- Begin function compare_and_swap
	.p2align	2
_compare_and_swap:                      ; @compare_and_swap
	.cfi_startproc
; %bb.0:
	mov	x8, x1
	casal	x8, x2, [x0]
	cmp	x8, x1
	cset	w0, eq
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
