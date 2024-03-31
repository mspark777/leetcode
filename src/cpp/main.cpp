#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
 public:
  long long countSubarrays(std::vector<int>& nums, int minK, int maxK) {
    long long result = 0;
    int badIdx = -1;
    int leftIdx = -1;
    int rightIdx = -1;

    for (int i = 0; i < static_cast<int>(nums.size()); i += 1) {
      const int num = nums[i];
      if ((num < minK) || (num > maxK)) {
        badIdx = i;
      }

      if (num == minK) {
        leftIdx = i;
      }

      if (num == maxK) {
        rightIdx = i;
      }

      result += std::max(0, std::min(leftIdx, rightIdx) - badIdx);
    }

    return result;
  }
};

struct Input {
  std::vector<int> nums;
  int minK;
  int maxK;
};

int main() {
  const Input inputs[] = {{{1, 3, 5, 2, 7, 5}, 1, 5}, {{1, 1, 1, 1}, 1, 1}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.countSubarrays(input.nums, input.minK, input.maxK)
              << std::endl;
  }
  return 0;
}
