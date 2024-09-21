#include "./main.h"

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *lexicalOrder(int n, int *return_size)
{
	int *result = malloc(sizeof(int) * n);
	*return_size = n;

	int cur = 1;

	for (int i = 0; i < n; i += 1) {
		result[i] = cur;

		if ((cur * 10) <= n) {
			cur *= 10;
			continue;
		}

		while (((cur % 10) == 9) || (cur >= n)) {
			cur /= 10;
		}

		cur += 1;
	}

	return result;
}
