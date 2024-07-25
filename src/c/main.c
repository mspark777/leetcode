#include "./main.h"

void merge(int *nums, int *buf, int left, int mid, int right)
{
	int l = left;
	int r = mid + 1;
	int i = 0;

	while (l <= mid && r <= right) {
		if (nums[l] <= nums[r]) {
			buf[i] = nums[l];
			l += 1;
			i += 1;
		} else {
			buf[i] = nums[r];
			i += 1;
			r += 1;
		}
	}

	while (l <= mid) {
		buf[i] = nums[l];
		i += 1;
		l += 1;
	}

	while (r <= right) {
		buf[i] = nums[r];
		i += 1;
		r += 1;
	}

	for (int j = 0; j < i; j += 1) {
		nums[left + j] = buf[j];
	}
}

void merge_sort_impl(int *nums, int *buf, int left, int right)
{
	if (left >= right) {
		return;
	}

	const int mid = (left + right) / 2;
	merge_sort_impl(nums, buf, left, mid);
	merge_sort_impl(nums, buf, mid + 1, right);
	merge(nums, buf, left, mid, right);
}

void merge_sort(int *nums, int left, int right)
{
	const int size = right - left + 1;
	int *buf = malloc(sizeof(int) * size);
	merge_sort_impl(nums, buf, left, right);
}

int *sortArray(int *nums, int nums_size, int *return_size)
{
	*return_size = nums_size;
	const int size = sizeof(int) * nums_size;
	int *result = malloc(size);
	memcpy(result, nums, size);
	merge_sort(result, 0, nums_size - 1);

	return result;
}
