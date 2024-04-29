#include <iostream>
#include <vector>

class Solution {
 public:
  int minOperations(std::vector<int>& nums, int k) {
    int result = 0;
    for (int n : nums) {
      result ^= n;
    }

    return __builtin_popcount(result ^ k);
  }
};

struct Input {
  std::vector<int> nums;
  int k;
};

int main() {
  const Input inputs[] = {
      {{2, 1, 3, 4}, 1},
      {{2, 0, 2, 0}, 0},
  };

  for (auto input : inputs) {
    Solution s;
    const auto result = s.minOperations(input.nums, input.k);
    std::cout << result << std::endl;
  }

  return 0;
}
