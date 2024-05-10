#include <stdio.h>
#include <stdlib.h>

int *kthSmallestPrimeFraction(int *arr, int arr_size, int k, int *return_size)
{
	double left = 0.0;
	double right = 1.0;
	while (left < right) {
		const double mid = (left + right) / 2.0;
		double max_fraction = 0.0;
		int numerator_idx = 0;
		int denominator_idx = 0;
		int kth_smaller = 0;
		int j = 1;
		for (int i = 0; i < arr_size - 1; i += 1) {
			while (1) {
				if (j >= arr_size) {
					break;
				}

				const double ai = (double)arr[i];
				const double aj = (double)arr[j];
				if (ai < (mid * aj)) {
					break;
				} else {
					j += 1;
				}
			}

			kth_smaller += arr_size - j;
			if (j == arr_size) {
				break;
			}

			const double fraction =
				((double)arr[i]) / ((double)arr[j]);
			if (fraction > max_fraction) {
				numerator_idx = i;
				denominator_idx = j;
				max_fraction = fraction;
			}
		}

		if (kth_smaller == k) {
			int *result = malloc(sizeof(int) * 2);
			result[0] = arr[numerator_idx];
			result[1] = arr[denominator_idx];
			*return_size = 2;
			return result;
		} else if (kth_smaller > k) {
			right = mid;
		} else {
			left = mid;
		}
	}

	*return_size = 0;
	return NULL;
}

int main()
{
	int return_size = 0;

	int arr0[] = { 1, 2, 3, 5 };
	int *result0 = kthSmallestPrimeFraction(arr0, 4, 3, &return_size);
	printf("%d %d\n", result0[0], result0[1]);

	int arr1[] = { 1, 7 };
	int *result1 = kthSmallestPrimeFraction(arr1, 2, 1, &return_size);
	printf("%d %d\n", result1[0], result1[1]);
}
