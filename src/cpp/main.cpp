#include <iostream>
#include <string>

class Solution {
 public:
  bool checkValidString(std::string s) {
    int leftCount = 0;
    int rightCount = 0;
    for (int i = 0; i < static_cast<int>(s.length()); i += 1) {
      const char left = s.at(i);
      const char right = s.at(s.length() - i - 1);

      if ((left == '(') || (left == '*')) {
        leftCount += 1;
      } else {
        leftCount -= 1;
      }

      if ((right == ')') || (right == '*')) {
        rightCount += 1;
      } else {
        rightCount -= 1;
      }

      if ((leftCount < 0) || (rightCount < 0)) {
        return false;
      }
    }

    return true;
  }
};

struct Input {
  std::string s;
};

int main() {
  const Input inputs[] = {{"()"}, {"(*)"}, {"(*))"}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.checkValidString(input.s) << std::endl;
  }
  return 0;
}
