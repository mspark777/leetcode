#include <stdio.h>

int findKthPositive(int *arr, int arr_size, int k) {
  int left = 0;
  int right = arr_size;

  while (left < right) {
    const int middle = (left + right) / 2;
    const int n = arr[middle] - (middle + 1);
    if (n < k) {
      left = middle + 1;
    } else {
      right = middle;
    }
  }
  return left + k;
}

int problem0(void) {
  int arr[] = {2, 3, 4, 7, 11};
  const int arr_size = sizeof(arr) / sizeof(arr[0]);
  const int k = 5;
  const int result = findKthPositive(arr, arr_size, k);
  return result;
}

int problem1(void) {
  int arr[] = {1, 2, 3, 4};
  const int arr_size = sizeof(arr) / sizeof(arr[0]);
  const int k = 2;
  const int result = findKthPositive(arr, arr_size, k);
  return result;
}

typedef int (*problem_t)(void);
int main() {
  const problem_t problems[] = {problem0, problem1};

  for (unsigned long i = 0; i < sizeof(problems) / sizeof(problems[0]);
       i += 1) {
    const int result = problems[i]();
    printf("%d\n", result);
  }
  return 0;
}
