#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

class Solution {
public:
  std::string mergeAlternately(std::string word1, std::string word2) {
    const int len1 = word1.length();
    const int len2 = word2.length();
    const int maxlen = std::max(len1, len2);
    const int rlen = len1 + len2;
    std::string result;
    result.reserve(rlen);

    auto iter1 = word1.begin();
    auto iter2 = word2.begin();
    for (int i = 0; i < maxlen; i += 1) {
      if (iter1 != word1.end()) {
        result.push_back(*iter1);
        ++iter1;
      }

      if (iter2 != word2.end()) {
        result.push_back(*iter2);
        ++iter2;
      }
    }

    return result;
  }
};

struct Input {
  std::string word1;
  std::string word2;
};

int main() {
  const Input inputs[] = {{"abc", "pqr"}, {"ab", "pqrs"}, {"abcd", "pq"}};

  for (auto input : inputs) {
    Solution solution;
    const auto result = solution.mergeAlternately(input.word1, input.word2);
    std::cout << result << std::endl;
  }
}
