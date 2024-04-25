#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
 public:
  int longestIdealString(std::string s, int k) {
    const int N = std::size(s);
    const int A_CODE = static_cast<int>('a');
    const int RANGE = static_cast<int>('z') - A_CODE + 1;
    std::vector<int> dp(RANGE, 0);

    int result = 0;
    for (int i = 0; i < N; i += 1) {
      const int curr = static_cast<int>(s.at(i)) - A_CODE;
      int best = 0;
      for (int prev = 0; prev < RANGE; prev += 1) {
        if (std::abs(prev - curr) <= k) {
          best = std::max(best, dp[prev]);
        }
      }

      dp[curr] = std::max(dp[curr], best + 1);
      result = std::max(result, dp[curr]);
    }

    return result;
  }
};

struct Input {
  std::string s;
  int k;
};

int main() {
  const Input inputs[] = {{"acfgbd", 2}, {"abcd", 3}};

  for (auto input : inputs) {
    Solution s;
    const auto result = s.longestIdealString(input.s, input.k);

    std::cout << result << std::endl;
  }

  return 0;
}
