#include <algorithm>
#include <iostream>
#include <string>

class Solution {
 public:
  int maxDepth(std::string s) {
    int result = 0;
    int depth = 0;

    for (char ch : s) {
      if (ch == '(') {
        depth += 1;
      } else if (ch == ')') {
        depth -= 1;
      }

      result = std::max(result, depth);
    }

    return result;
  }
};

struct Input {
  std::string s;
};

int main() {
  const Input inputs[] = {{"(1+(2*3)+((8)/4))+1"}, {"(1)+((2))+(((3)))"}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.maxDepth(input.s) << std::endl;
  }
  return 0;
}
