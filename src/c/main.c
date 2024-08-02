#include "./main.h"

int minSwaps(int *nums, int nums_size)
{
	int result = INT_MAX;
	int total_ones = 0;
	for (int i = 0; i < nums_size; i += 1) {
		total_ones += nums[i];
	}

	int ones_count = nums[0];
	int end = 0;

	for (int i = 0; i < nums_size; i += 1) {
		if (i > 0) {
			ones_count -= nums[i - 1];
		}

		while ((end - i + 1) < total_ones) {
			end += 1;
			ones_count += nums[end % nums_size];
		}

		const int ones = total_ones - ones_count;
		if (ones < result) {
			result = ones;
		}
	}

	return result;
}
