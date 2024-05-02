#include <bitset>
#include <cstdlib>
#include <iostream>
#include <vector>

class Solution {
 public:
  int findMaxK(std::vector<int>& nums) {
    int result = -1;
    std::bitset<2048> seen;

    for (const int num : nums) {
      const int anum = std::abs(num);
      if (anum > result && seen[1024 - num]) {
        result = anum;
      }

      seen[num + 1024] = true;
    }

    return result;
  }
};

struct Input {
  std::vector<int> nums;
};

int main() {
  const Input inputs[] = {
      {{-1, 2, -3, 3}}, {{-1, 10, 6, 7, -7, 1}}, {{-10, 8, 6, 7, -2, -3}}};

  for (auto input : inputs) {
    Solution s;
    const auto result = s.findMaxK(input.nums);
    std::cout << result << std::endl;
  }

  return 0;
}
