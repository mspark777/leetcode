#include <iostream>
#include <string>

class Solution {
 public:
  int lengthOfLastWord(std::string s) {
    int result = 0;
    for (auto iter = s.rbegin(); iter != s.rend(); ++iter) {
      const char ch = *iter;
      if (ch == ' ') {
        if (result > 0) {
          return result;
        } else {
          continue;
        }
      } else {
        result += 1;
      }
    }

    return result;
  }
};

struct Input {
  std::string s;
};

int main() {
  const Input inputs[] = {{"Hello World"},
                          {"   fly me   to   the moon  "},
                          {"luffy is still joyboy"}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.lengthOfLastWord(input.s) << std::endl;
  }
  return 0;
}
