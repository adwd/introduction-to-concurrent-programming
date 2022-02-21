	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0	sdk_version 12, 1
	.globl	_test_and_set                   ; -- Begin function test_and_set
	.p2align	2
_test_and_set:                          ; @test_and_set
	.cfi_startproc
; %bb.0:
	mov	w8, #1
	swpalb	w8, w8, [x0]
	and	w0, w8, #0x1
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_tas_release                    ; -- Begin function tas_release
	.p2align	2
_tas_release:                           ; @tas_release
	.cfi_startproc
; %bb.0:
	stlrb	wzr, [x0]
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
