int min(int left, int right)
{
	return left < right ? left : right;
}

int max(int left, int right)
{
	return left < right ? right : left;
}

long long maximumValueSum(int *nums, int nums_size, int k, int **edges,
			  int edgesSize, int *edgesColSize)
{
	long long sum = 0;
	int count = 0;
	int positive_minimum = 1 << 30;
	int negative_maximum = -positive_minimum;

	for (int i = 0; i < nums_size; i += 1) {
		const int num = nums[i];
		const int operated_node_value = num ^ k;

		sum += num;
		const int net_change = operated_node_value - num;
		if (net_change > 0) {
			positive_minimum = min(positive_minimum, net_change);
			sum += net_change;
			count += 1;
		} else {
			negative_maximum = max(negative_maximum, net_change);
		}
	}

	if ((count & 1) == 0) {
		return sum;
	}

	const long long with_positive = sum - positive_minimum;
	const long long with_negative = sum + negative_maximum;
	return with_positive > with_negative ? with_positive : with_negative;
}
