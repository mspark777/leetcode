#include "./main.h"

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *missingRolls(int *rolls, int rolls_size, int mean, int n, int *return_size)
{
	int sum = 0;
	for (int i = 0; i < rolls_size; i += 1) {
		sum += rolls[i];
	}

	int remaining_sum = mean * (n + rolls_size) - sum;

	if (remaining_sum > (6 * n)) {
		*return_size = 0;
		return malloc(sizeof(int));
	} else if (remaining_sum < n) {
		*return_size = 0;
		return malloc(sizeof(int));
	}

	int distribute_mean = remaining_sum / n;
	int mod = remaining_sum % n;

	int *result = malloc(sizeof(int) * n);
	*return_size = n;

	for (int i = 0; i < mod; i += 1) {
		result[i] = distribute_mean + 1;
	}

	for (int i = mod; i < n; i += 1) {
		result[i] = distribute_mean;
	}

	return result;
}
