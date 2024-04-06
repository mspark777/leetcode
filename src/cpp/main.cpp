#include <iostream>
#include <stack>
#include <string>

class Solution {
 public:
  std::string minRemoveToMakeValid(std::string s) {
    int leftCount = 0;
    int rightCount = 0;
    std::stack<char> stack;

    for (char ch : s) {
      leftCount += ch == '(';
      rightCount += ch == ')';

      if (rightCount > leftCount) {
        rightCount -= 1;
      } else {
        stack.push(ch);
      }
    }

    std::string result;
    result.reserve(s.size());

    while (!stack.empty()) {
      const char ch = stack.top();
      stack.pop();
      if (leftCount > rightCount && ch == '(') {
        leftCount -= 1;
      } else {
        result += ch;
      }
    }

    return std::string(result.rbegin(), result.rend());
  }
};

struct Input {
  std::string s;
};

int main() {
  const Input inputs[] = {{"lee(t(c)o)de)"}, {"a)b(c)d"}, {"))(("}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.minRemoveToMakeValid(input.s) << std::endl;
  }
  return 0;
}
