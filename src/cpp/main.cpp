#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
 public:
  long long countSubarrays(std::vector<int>& nums, int k) {
    const int maxNum = *std::max_element(nums.begin(), nums.end());
    long long result = 0;
    int start = 0;

    for (int end = 0; end < static_cast<int>(nums.size()); end += 1) {
      if (nums[end] == maxNum) {
        k -= 1;
      }

      while (k == 0) {
        if (nums[start] == maxNum) {
          k = +1;
        }

        start += 1;
      }
      result += start;
    }

    return result;
  }
};

struct Input {
  std::vector<int> nums;
  int k;
};

int main() {
  const Input inputs[] = {{{1, 3, 2, 3, 3}, 2}, {{1, 4, 2, 1}, 3}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.countSubarrays(input.nums, input.k) << std::endl;
  }
  return 0;
}
