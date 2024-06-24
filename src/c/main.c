#define static_cast(t, v) ((t)v)

int minKBitFlips(int *nums, int nums_size, int k)
{
	int current_flips = 0;
	int result = 0;

	for (int i = 0; i < nums_size; i += 1) {
		if ((i >= k) && (nums[i - k] == 2)) {
			current_flips -= 1;
		}

		if ((current_flips & 1) == nums[i]) {
			if ((i + k) > nums_size) {
				return -1;
			}

			nums[i] = 2;
			current_flips += 1;
			result += 1;
		}
	}

	return result;
}
