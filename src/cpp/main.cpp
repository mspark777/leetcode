#include <cmath>
#include <cstddef>
#include <iostream>
#include <vector>

class Solution {
 public:
  int numSubarrayProductLessThanK(std::vector<int>& nums, int k) {
    if (k == 0) {
      return 0;
    }

    const std::size_t m = nums.size() + 1;
    std::vector<double> logsPrefixSum(m);

    for (std::size_t i = 0; i < nums.size(); i += 1) {
      logsPrefixSum[i + 1] =
          logsPrefixSum[i] + log(static_cast<double>(nums[i]));
    }

    const double logK = log(k);
    int result = 0;
    for (int currIdx = 0; currIdx < static_cast<int>(m); currIdx += 1) {
      int low = currIdx + 1;
      int high = m;
      while (low < high) {
        const int mid = (low + high) / 2;
        if (logsPrefixSum[mid] < logsPrefixSum[currIdx] + logK - 1e-9) {
          low = mid + 1;
        } else {
          high = mid;
        }
      }
      result += low - currIdx - 1;
    }

    return result;
  }
};

struct Input {
  std::vector<int> nums;
  int k;
};

int main() {
  const Input inputs[] = {
      {{10, 5, 2, 6}, 100},
      {{1, 2, 3}, 0},
  };

  for (auto input : inputs) {
    Solution s;
    std::cout << s.numSubarrayProductLessThanK(input.nums, input.k)
              << std::endl;
  }
  return 0;
}
