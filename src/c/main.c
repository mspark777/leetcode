#include "./main.h"

int maxCount(int m, int n, int **ops, int ops_size, int *ops_col_size)
{
	int r = m;
	int c = n;

	for (int i = 0; i < ops_size; i += 1) {
		const int *op = ops[i];
		const int a = op[0];
		const int b = op[1];
		r = a < r ? a : r;
		c = b < c ? b : c;
	}

	return r * c;
}
