#include "./main.h"

bool hasAlternatingBits(int n)
{
	long long n0 = n;
	long long n1 = n >> 1;
	long long n2 = n0 + n1;
	long long n3 = n2 + 1;

	return (n2 & n3) == 0;
}
