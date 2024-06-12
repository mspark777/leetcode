void swap(int *left, int *right)
{
	const int l = *left;
	const int r = *right;
	*left = r;
	*right = l;
}

void sortColors(int *nums, int nums_size)
{
	const int RED = 0;
	const int WHITE = 1;
	int left = 0;
	int mid = 0;
	int right = nums_size - 1;
	while (mid <= right) {
		const int num = nums[mid];
		switch (num) {
		case RED:
			swap(nums + left, nums + mid);
			left += 1;
			mid += 1;
			break;
		case WHITE:
			mid += 1;
			break;
		default:
			swap(nums + mid, nums + right);
			right -= 1;
		}
	}
}
