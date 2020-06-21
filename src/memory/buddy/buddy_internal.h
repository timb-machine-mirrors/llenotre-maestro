#ifndef BUDDY_INTERNAL_H
# define BUDDY_INTERNAL_H

# include <memory/buddy/buddy.h>
# include <util/util.h>

# define BUDDY_BLOCK_OFFSET(ptr)	((uintptr_t) (ptr)\
	- (uintptr_t) mem_info.heap_begin)
# define BUDDY_ADDR(ptr, order)		((void *) (mem_info.heap_begin\
	+ (BUDDY_BLOCK_OFFSET(ptr) ^ BLOCK_SIZE(order))))

typedef struct buddy_free_block
{
	/* Double-linked list of free blocks of the same order. */
	list_head_t free_list;
	/* The AVL tree node. */
	avl_tree_t node;

	/* The block's order. */
	block_order_t order;
} buddy_free_block_t;

#endif