#include <stdio.h>

int firstMissingPositive(int* nums, int numsSize) {
  for (int i = 0; i < numsSize; i += 1) {
    while ((nums[i] > 0) && (nums[i] <= numsSize) &&
           (nums[nums[i] - 1] != nums[i])) {
      int temp = nums[i];
      nums[i] = nums[temp - 1];
      nums[temp - 1] = temp;
    }
  }

  for (int i = 0; i < numsSize; i += 1) {
    if (nums[i] != i + 1) {
      return i + 1;
    }
  }

  return numsSize + 1;
}

int main() {
  int input0[] = {1, 2, 0};
  printf("%d\n", firstMissingPositive(input0, 3));

  int input1[] = {3, 4, -1, 1};
  printf("%d\n", firstMissingPositive(input1, 4));

  int input2[] = {7, 8, 9, 11, 12};
  printf("%d\n",
         firstMissingPositive(input2, sizeof(input2) / sizeof(input2[0])));

  return 0;
}
