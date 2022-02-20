	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 12, 0	sdk_version 12, 1
	.globl	_compare_and_swap               ## -- Begin function compare_and_swap
	.p2align	4, 0x90
_compare_and_swap:                      ## @compare_and_swap
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movq	(%rdi), %rax
	cmpq	%rsi, %rax
	jne	LBB0_2
## %bb.1:
	movq	%rdx, (%rdi)
LBB0_2:
	cmpq	%rsi, %rax
	sete	%al
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
.subsections_via_symbols
