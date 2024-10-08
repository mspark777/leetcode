#include "./main.h"

int findLengthOfLCIS(int *nums, int nums_size)
{
	int result = 1;
	int len = 1;
	for (int i = 1; i < nums_size; i += 1) {
		const int prev = nums[i - 1];
		const int curr = nums[i];
		if (prev < curr) {
			len += 1;
		} else {
			if (len > result) {
				result = len;
			}
			len = 1;
		}
	}

	if (len > result) {
		result = len;
	}

	return result;
}
