#include <stdio.h>

int partition(int *arr, int left, int right)
{
	int pivot_idx = -1;
	for (int i = left; i < right; i += 1) {
		if (arr[i] < arr[i + 1]) {
			pivot_idx = i;
			break;
		}
	}

	if (pivot_idx < 0) {
		return pivot_idx;
	}

	const int pivot = arr[pivot_idx];
	arr[pivot_idx] = arr[right];
	arr[right] = pivot;

	int l = left - 1;
	for (int r = left; r < right; r += 1) {
		const int n = arr[r];
		if (n > pivot) {
			l += 1;
			arr[r] = arr[l];
			arr[l] = n;
		}
	}

	const int next_pivot_idx = l + 1;
	arr[right] = arr[next_pivot_idx];
	arr[next_pivot_idx] = pivot;
	return next_pivot_idx;
}

void sort(int *arr, int left, int right)
{
	if (left < right) {
		const int pivot = partition(arr, left, right);
		if (pivot >= 0) {
			sort(arr, left, pivot - 1);
			sort(arr, pivot + 1, right);
		}
	}
}

long long maximumHappinessSum(int *happiness, int happinessSize, int k)
{
	sort(happiness, 0, happinessSize - 1);

	long long result = 0;
	for (int turn = 0; turn < k; turn += 1) {
		const int value = happiness[turn] - turn;
		if (value > 0) {
			result += value;
		} else {
			break;
		}
	}

	return result;
}

int main()
{
	int happiness0[] = { 1, 2, 3 };
	printf("%lld\n", maximumHappinessSum(happiness0, 3, 2));

	int happiness1[] = { 1, 1, 1, 1 };
	printf("%lld\n", maximumHappinessSum(happiness1, 4, 2));

	int happiness2[] = { 2, 3, 4, 5 };
	printf("%lld\n", maximumHappinessSum(happiness2, 4, 1));

	int happiness3[] = { 7, 2, 28 };
	printf("%lld\n", maximumHappinessSum(happiness3, 3, 2));
}
