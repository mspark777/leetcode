#include "./main.h"

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *xorQueries(int *arr, int arr_size, int **queries, int queries_size,
		int *queries_col_size, int *return_size)
{
	for (int i = 1; i < arr_size; i += 1) {
		arr[i] ^= arr[i - 1];
	}

	*return_size = queries_size;
	int *result = malloc(sizeof(int) * queries_size);
	for (int i = 0; i < queries_size; i += 1) {
		const int l = queries[i][0];
		const int r = queries[i][1];

		if (l > 0) {
			result[i] = arr[l - 1] ^ arr[r];
		} else {
			result[i] = arr[r];
		}
	}

	return result;
}
