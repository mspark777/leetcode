#include <stdio.h>

void swap(int *left, int *right)
{
	const int temp_left = *left;
	const int temp_right = *right;
	*left = temp_right;
	*right = temp_left;
}

int partition(int *arr, int left, int right)
{
	const int pivot = arr[right];
	int i = left - 1;

	for (int j = left; j <= right; j += 1) {
		if (arr[j] < pivot) {
			i += 1;
			swap(arr + i, arr + j);
		}
	}

	swap(arr + i + 1, arr + right);
	return i + 1;
}

void sort1(int *arr, int left, int right)
{
	if (left < right) {
		const int pivot = partition(arr, left, right);
		sort1(arr, left, pivot - 1);
		sort1(arr, pivot + 1, right);
	}
}

void sort(int *arr, const int size)
{
	sort1(arr, 0, size - 1);
}

int numRescueBoats(int *people, const int people_size, const int limit)
{
	sort(people, people_size);

	int left = 0;
	int right = people_size - 1;
	int result = 0;

	while (left <= right) {
		const int light = people[left];
		const int heavy = people[right];
		result += 1;
		right -= 1;
		const int total = light + heavy;
		if (total <= limit) {
			left += 1;
		}
	}

	return result;
}

int main()
{
	int people0[] = { 1, 2 };
	printf("%d\n", numRescueBoats(people0, 2, 3));

	int people1[] = { 3, 2, 2, 1 };
	printf("%d\n", numRescueBoats(people1, 4, 3));

	int people2[] = { 3, 5, 3, 4 };
	printf("%d\n", numRescueBoats(people2, 4, 5));
}
