#include "./main.h"

int count_steps(long long n, long long prefix1, long long prefix2)
{
	long long n1 = n + 1;
	long long steps = 0;
	while (prefix1 <= n) {
		steps += (n1 < prefix2 ? n1 : prefix2) - prefix1;
		prefix1 *= 10;
		prefix2 *= 10;
	}

	return steps;
}

int findKthNumber(int n, int k)
{
	long long result = 1;
	k -= 1;

	while (k > 0) {
		int step = count_steps(n, result, result + 1);
		if (step <= k) {
			result += 1;
			k -= step;
		} else {
			result *= 10;
			k -= 1;
		}
	}

	return result;
}
