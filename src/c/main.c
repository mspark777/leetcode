#define static_cast(t, v) ((t)v)

int minPatches(int *nums, int nums_size, int n)
{
	const long long N = n;
	long long missing = 1;
	int result = 0;
	int i = 0;

	while (missing <= N) {
		if (i < nums_size &&
		    static_cast(long long, nums[i]) <= missing) {
			missing += nums[i];
			i += 1;
		} else {
			missing += missing;
			result += 1;
		}
	}

	return result;
}
