#include "./main.h"

int longestSubarray(int *nums, int nums_size)
{
	int max_num = 0;
	int result = 0;
	int current = 0;

	for (int i = 0; i < nums_size; i += 1) {
		int num = nums[i];
		if (max_num < num) {
			max_num = num;
			result = 0;
			current = 0;
		}

		if (max_num == num) {
			current += 1;
		} else {
			current = 0;
		}

		if (result < current) {
			result = current;
		}
	}

	return result;
}
