#include <stdio.h>
#include <stdlib.h>

#define bool int

int search(const int *nums, const int size, const int num) {
  int left = 0;
  int right = size - 1;

  while (left <= right) {
    const int mid = (right + left) / 2;
    if (nums[mid] < num) {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }

  return left;
}

int lengthOfLIS(int *nums, int nums_size) {
  int *temp = calloc(nums_size, sizeof(int));
  int result = 0;

  temp[0] = nums[0];

  for (int i = 0; i < nums_size; i += 1) {
    const int num = nums[i];
    if (num > temp[result]) {
      result += 1;
      temp[result] = num;
    } else {
      const int index = search(temp, result + 1, num);
      temp[index] = num;
    }
  }

  free(temp);
  temp = NULL;

  return result + 1;
}

int main() {
  int nums0[] = {10, 9, 2, 5, 3, 7, 101, 18};
  printf("%d\n", lengthOfLIS(nums0, sizeof(nums0) / sizeof(nums0[0])));

  int nums1[] = {0, 1, 0, 3, 2, 3};
  printf("%d\n", lengthOfLIS(nums1, sizeof(nums1) / sizeof(nums1[0])));

  int nums2[] = {7, 7, 7, 7, 7, 7, 7};
  printf("%d\n", lengthOfLIS(nums2, sizeof(nums2) / sizeof(nums2[0])));
  return 0;
}
