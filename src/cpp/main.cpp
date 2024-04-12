#include <algorithm>
#include <iostream>
#include <map>
#include <vector>

class Solution {
 public:
  int findLHS(std::vector<int>& nums) {
    int result = 0;
    std::map<int, int> counts;

    for (int n : nums) {
      counts[n] += 1;
    }

    for (int i : nums) {
      const int j = i + 1;
      if (counts.find(j) != counts.end()) {
        const int count = counts[i] + counts[j];
        result = std::max(count, result);
      }
    }

    return result;
  }
};

struct Input {
  std::vector<int> nums;
};

int main() {
  const Input inputs[] = {
      {{1, 3, 2, 2, 5, 2, 3, 7}}, {{1, 2, 3, 4}}, {{1, 1, 1, 1}}

  };

  for (auto input : inputs) {
    Solution s;
    const auto result = s.findLHS(input.nums);
    std::cout << result << std::endl;
  }

  return 0;
}
