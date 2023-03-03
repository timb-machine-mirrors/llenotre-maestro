// C functions utils

#ifndef LIBC_H
# define LIBC_H

// Gives the offset of the pointer `ptr` relative to its down-aligned
// counterpart.
# define ALIGN_MASK(ptr, n)	((intptr_t) (ptr) & ((n) - 1))

// Tells whether the pointer `ptr` is aligned on boundary `n`.
//
// If `n` is zero, the behaviour is undefined.
# define IS_ALIGNED(ptr, n)	(ALIGN_MASK(ptr, n) == 0)

// Aligns down the given memory pointer `ptr` to the boundary `n`.
//
// If `n` is zero, the behaviour is undefined.
# define DOWN_ALIGN(ptr, n)\
	(typeof(ptr)) ((intptr_t) (ptr) & ~((intptr_t) ((n) - 1)))

#endif
