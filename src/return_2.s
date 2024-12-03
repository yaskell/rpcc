	.file	"return_2.c"
	.text
	.globl	main
	.type	main, @function
main:
	movl	$2, %eax
	ret
	.size	main, .-main
	.ident	"GCC: (GNU) 13.2.0"
	.section	.note.GNU-stack,"",@progbits
