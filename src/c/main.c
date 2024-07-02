#include "./main.h"

int *intersect(int *nums1, int nums1_size, int *nums2, int nums2_size,
	       int *return_size)
{
	int *counts = calloc(1001, sizeof(int));
	for (int i = 0; i < nums1_size; i += 1) {
		const int num = nums1[i];
		counts[num] += 1;
	}

	int j = 0;
	int *result = calloc(nums1_size < nums2_size ? nums1_size : nums2_size,
			     sizeof(int));

	for (int i = 0; i < nums2_size; i += 1) {
		const int num = nums2[i];
		const int count = counts[num];
		if (count > 0) {
			counts[num] -= 1;
			result[j] = num;
			j += 1;
		}
	}

	*return_size = j;
	return result;
}
