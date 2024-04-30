#include <iostream>
#include <map>
#include <string>

class Solution {
 public:
  long long wonderfulSubstrings(std::string word) {
    const int N = word.length();
    std::map<int, int> frequencies;
    frequencies[0] = 1;

    int mask = 0;
    long long result = 0;
    for (int i = 0; i < N; i += 1) {
      const char ch = word.at(i);
      const int bit = ch - 'a';
      mask ^= 1 << bit;
      const int frequency = frequencies[mask];
      result += frequency;
      frequencies[mask] = frequency + 1;
      for (int odd = 0; odd < 10; odd += 1) {
        result += frequencies[mask ^ (1 << odd)];
      }
    }

    return result;
  }
};

struct Input {
  std::string word;
};

int main() {
  const Input inputs[] = {{"aba"}, {"aabb"}, {"he"}};

  for (auto input : inputs) {
    Solution s;
    const auto result = s.wonderfulSubstrings(input.word);
    std::cout << result << std::endl;
  }

  return 0;
}
