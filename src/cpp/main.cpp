#include <iostream>
#include <unordered_map>
#include <vector>

class Solution {
 public:
  int maxSubarrayLength(std::vector<int>& nums, int k) {
    const int n = static_cast<int>(nums.size());
    std::unordered_map<int, int> frequencies;
    int start = 0;
    int charsWithFreqOverK = 0;

    for (int end = 0; end < n; end += 1) {
      const int n = nums[end];
      frequencies[n] += 1;
      if (frequencies[n] == k + 1) {
        charsWithFreqOverK += 1;
      }

      if (charsWithFreqOverK > 0) {
        const int s = nums[start];
        frequencies[s] -= 1;
        if (frequencies[s] == k) {
          charsWithFreqOverK -= 1;
        }
        start += 1;
      }
    }

    return n - start;
  }
};

struct Input {
  std::vector<int> nums;
  int k;
};

int main() {
  const Input inputs[] = {{{1, 2, 3, 1, 2, 3, 1, 2}, 2},
                          {{1, 2, 1, 2, 1, 2, 1, 2}, 1},
                          {{5, 5, 5, 5, 5, 5, 5}, 4}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.maxSubarrayLength(input.nums, input.k) << std::endl;
  }
  return 0;
}
