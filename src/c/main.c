#define static_cast(t, v) ((t)v)

int at_most(int *nums, int nums_size, int k)
{
	int window_size = 0;
	int subarrays = 0;
	int start = 0;

	for (int end = 0; end < nums_size; end += 1) {
		window_size += nums[end] & 1;
		while (window_size > k) {
			window_size -= nums[start] & 1;
			start += 1;
		}

		subarrays += end - start + 1;
	}

	return subarrays;
}

int numberOfSubarrays(int *nums, int nums_size, int k)
{
	return at_most(nums, nums_size, k) - at_most(nums, nums_size, k - 1);
}
