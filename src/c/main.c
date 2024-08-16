#include "./main.h"

int maxDistance(int **arrays, int arrays_size, int *arrays_col_size)
{
	int min = 1e5;
	int max = -1e5;
	int result = 0;

	for (int i = 0; i < arrays_size; i += 1) {
		int *arr = arrays[i];
		int left = arr[0];
		int right = arr[arrays_col_size[i] - 1];
		int min_diff = right - min;
		int max_diff = max - left;

		if (min_diff > result) {
			result = min_diff;
		}

		if (max_diff > result) {
			result = max_diff;
		}

		if (left < min) {
			min = left;
		}

		if (right > max) {
			max = right;
		}
	}

	return result;
}
