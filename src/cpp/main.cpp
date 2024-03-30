#include <iostream>
#include <vector>

class Solution {
 public:
  int subarraysWithKDistinct(std::vector<int>& nums, int k) {
    std::vector<int> distinctCounts(nums.size() + 1, 0);

    int result = 0;
    int left = 0;
    int right = 0;
    int currCount = 0;

    while (right < static_cast<int>(nums.size())) {
      const int r = nums[right];
      right += 1;
      distinctCounts[r] += 1;
      if (distinctCounts[r] == 1) {
        k -= 1;
      }

      if (k < 0) {
        distinctCounts[nums[left]] -= 1;
        left += 1;
        k += 1;
        currCount = 0;
      }

      if (k == 0) {
        while (distinctCounts[nums[left]] > 1) {
          distinctCounts[nums[left]] -= 1;
          left += 1;
          currCount += 1;
        }
        result += currCount + 1;
      }
    }

    return result;
  }
};

struct Input {
  std::vector<int> nums;
  int k;
};

int main() {
  const Input inputs[] = {{{1, 2, 1, 2, 3}, 2}, {{1, 2, 1, 3, 4}, 3}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.subarraysWithKDistinct(input.nums, input.k) << std::endl;
  }
  return 0;
}
