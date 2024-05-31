#include <stdlib.h>

int *singleNumber(int *nums, int nums_size, int *return_size)
{
	long long diff = 0;
	for (int i = 0; i < nums_size; i += 1) {
		diff ^= nums[i];
	}

	diff &= -diff;

	int left = 0;
	int right = 0;
	for (int i = 0; i < nums_size; i += 1) {
		const int num = nums[i];
		if (num & diff) {
			right ^= num;
		} else {
			left ^= num;
		}
	}

	int *result = malloc(sizeof(int) * 2);
	result[0] = left;
	result[1] = right;
	*return_size = 2;

	return result;
}
