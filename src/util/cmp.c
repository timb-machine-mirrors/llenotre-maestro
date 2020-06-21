#include <util/util.h>

/*
 * Compares the given pointers.
 */
int ptr_cmp(const void *p0, const void *p1)
{
	return (uintptr_t) p1 - (uintptr_t) p0;
}

/*
 * Compares the avl values pointed by the given pointers.
 */
int avl_val_cmp(const void *v0, const void *v1)
{
	return (*(avl_value_t *) v1) - (*(avl_value_t *) v0);
}