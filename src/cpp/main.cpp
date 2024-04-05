#include <cstdlib>
#include <iostream>
#include <string>

class Solution {
 public:
  std::string makeGood(std::string s) {
    std::string result;
    result.reserve(s.size());

    for (char ch : s) {
      if (!result.empty()) {
        const char next = *result.rbegin();
        const int diff = static_cast<int>(std::abs(ch - next));
        if (diff == 32) {
          result.pop_back();
          continue;
        }
      }

      result.push_back(ch);
    }

    return result;
  }
};

struct Input {
  std::string s;
};

int main() {
  const Input inputs[] = {{"leEeetcode"}, {"abBAcC"}, {"s"}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.makeGood(input.s) << std::endl;
  }
  return 0;
}
