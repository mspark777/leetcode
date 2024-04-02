#include <iostream>
#include <map>
#include <string>

class Solution {
 public:
  bool isIsomorphic(std::string s, std::string t) {
    std::map<char, int> sIdxMap;
    std::map<char, int> tIdxMap;
    for (int i = 0; i < static_cast<int>(s.size()); i += 1) {
      const char sch = s[i];
      const char tch = t[i];

      if (sIdxMap.find(sch) == sIdxMap.end()) {
        sIdxMap[sch] = i;
      }
      if (tIdxMap.find(tch) == tIdxMap.end()) {
        tIdxMap[tch] = i;
      }

      if (sIdxMap[sch] != tIdxMap[tch]) {
        return false;
      }
    }

    return true;
  }
};

struct Input {
  std::string s;
  std::string t;
};

int main() {
  const Input inputs[] = {
      {"egg", "add"}, {"foo", "bar"}, {"paper", "title"}, {"badc", "baba"}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.isIsomorphic(input.s, input.t) << std::endl;
  }
  return 0;
}
