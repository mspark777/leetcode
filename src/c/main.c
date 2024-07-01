#include "./main.h"

bool threeConsecutiveOdds(int *arr, int arr_size)
{
	int count = 0;
	for (int i = 0; i < arr_size; i += 1) {
		if (arr[i] & 1) {
			count += 1;
		} else {
			count = 0;
		}

		if (count == 3) {
			break;
		}
	}

	return count == 3;
}
