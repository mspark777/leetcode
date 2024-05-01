#include <iostream>
#include <string>

class Solution {
 public:
  std::string reversePrefix(std::string word, char ch) {
    int right = word.find(ch);
    if (right == static_cast<int>(std::string::npos)) {
      return word;
    }

    int left = 0;
    while (left < right) {
      const char temp = word[left];
      word[left] = word[right];
      word[right] = temp;
      left += 1;
      right -= 1;
    }

    return word;
  }
};

struct Input {
  std::string word;
  char ch;
};

int main() {
  const Input inputs[] = {
      {"abcdefd", 'd'},
      {"xyxzxe", 'z'},
      {"abcd", 'z'},

  };

  for (auto input : inputs) {
    Solution s;
    const auto result = s.reversePrefix(input.word, input.ch);
    std::cout << result << std::endl;
  }

  return 0;
}
