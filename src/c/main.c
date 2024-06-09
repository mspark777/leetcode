#include <stdlib.h>

int subarraysDivByK(int *nums, int nums_size, int k)
{
	int prefix = 0;
	int result = 0;

	int *groups = calloc(sizeof(int), k);
	groups[0] = 1;

	for (int i = 0; i < nums_size; i += 1) {
		prefix = (prefix + k + (nums[i] % k)) % k;
		result += groups[prefix];
		groups[prefix] += 1;
	}

	free(groups);
	groups = NULL;

	return result;
}
