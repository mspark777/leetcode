int subsetXORSum(int *nums, int nums_size)
{
	int result = 0;

	for (int i = 0; i < nums_size; i += 1) {
		result |= nums[i];
	}

	return result << (nums_size - 1);
}
