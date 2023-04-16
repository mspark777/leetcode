#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

class Solution {
public:
  int longestPalindromeSubseq(std::string s) {
    const int n = s.size();
    std::vector<int> dp(n);
    std::vector<int> dp_prev(n);

    for (int i = n - 1; i >= 0; i -= 1) {
      dp[i] = 1;
      for (int j = i + 1; j < n; j += 1) {
        if (s[i] == s[j]) {
          dp[j] = dp_prev[j - 1] + 2;
        } else {
          dp[j] = std::max(dp_prev[j], dp[j - 1]);
        }
      }

      dp_prev = dp;
    }

    return dp[n - 1];
  }
};

int main() {
  std::string inputs[] = {"bbbab", "cbbd"};

  for (auto input : inputs) {
    Solution solution;
    const int result = solution.longestPalindromeSubseq(input);
    std::cout << result << std::endl;
  }
}
