#include "./main.h"

int chalkReplacer(int *chalk, int chalk_size, int k)
{
	int sum_chalk = 0;
	for (int i = 0; i < chalk_size; i += 1) {
		sum_chalk += chalk[i];
		if (sum_chalk > k) {
			break;
		}
	}

	k = k % sum_chalk;
	for (int i = 0; i < chalk_size; i += 1) {
		if (k < chalk[i]) {
			return i;
		}

		k -= chalk[i];
	}

	return 0;
}
