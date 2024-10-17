#include "./main.h"

int search(int *nums, int nums_size, int target)
{
	int left = 0;
	int right = nums_size;

	while (left < right) {
		const int mid = (left + right) / 2;
		const int num = nums[mid];
		if (num < target) {
			left = mid + 1;
		} else if (num > target) {
			right = mid;
		} else {
			return mid;
		}
	}

	return -1;
}
