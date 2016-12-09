#include        <basic.h>
#include        <bstdio.h>
#include        <btron/btron.h>

IMPORT W plus_one(W x);
IMPORT W minus_one(W x);
IMPORT W sample_call(W x);

typedef          long long di_int;
typedef unsigned long long du_int;
di_int __mulodi4(di_int a, di_int b, int* overflow)
{
	*overflow = 0;
	return 0;
}

du_int __udivdi3(du_int a, du_int b)
{
	return 0;
}

du_int __umoddi3(du_int a, du_int b)
{
    return 0;
}

EXPORT W link_confirmation(W value)
{
	return -1;
}

EXPORT W MAIN(MESSAGE *msg)
{
	printf("test: %d\n", plus_one(2));
	printf("test: %d\n", minus_one(2));
	printf("test: %d\n", sample_call(2));

	return 0;
}
