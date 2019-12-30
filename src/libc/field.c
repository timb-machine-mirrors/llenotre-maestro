#include <libc/libc_internal.h>
#include <libc/string.h>
#include <util/attr.h>

ATTR_HOT
ATTR_CONST
long make_field(const int c)
{
	long field = 0;
	size_t i = 0;

	for(; i < sizeof(long); ++i)
		field = (field << 1) | (c & 0xff);
	return field;
}
