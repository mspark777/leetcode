#include <iostream>
#include <vector>

class Solution {
 public:
  std::string removeKdigits(std::string num, int k) {
    std::vector<char> stack;
    stack.reserve(num.length());

    for (const char ch : num) {
      while (k > 0 && !stack.empty() && stack.back() > ch) {
        stack.pop_back();
        k -= 1;
      }

      stack.push_back(ch);
    }

    while (k > 0) {
      stack.pop_back();
      k -= 1;
    }

    std::string result;
    result.reserve(stack.size());

    for (const char ch : stack) {
      if (result.empty() && ch == '0') {
        continue;
      }

      result.push_back(ch);
    }

    return result.empty() ? "0" : result;
  }
};

struct Input {
  std::string num;
  int k;
};

int main() {
  const Input inputs[] = {{"1432219", 3}, {"10200", 1}, {"10", 2}};

  for (auto input : inputs) {
    Solution s;
    const auto result = s.removeKdigits(input.num, input.k);
    std::cout << result << std::endl;
  }

  return 0;
}
