#include <algorithm>
#include <iostream>
#include <queue>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

class Solution {
 public:
  int findRotateSteps(std::string ring, std::string key) {
    const int ringLen = std::size(ring);
    const int keyLen = std::size(key);
    std::unordered_map<char, std::vector<int>> indexMap;
    for (int i = 0; i < ringLen; i += 1) {
      const char ch = ring[i];
      indexMap[ch].push_back(i);
    }

    std::priority_queue<std::vector<int>, std::vector<std::vector<int>>,
                        std::greater<std::vector<int>>>
        heap;
    heap.push({0, 0, 0});
    std::unordered_set<std::string> seen;
    int totalSteps = 0;
    while (!heap.empty()) {
      std::vector<int> state = heap.top();
      heap.pop();
      totalSteps = state[0];
      int ringIndex = state[1];
      int keyIndex = state[2];
      if (keyIndex == keyLen) {
        break;
      }
      std::string currentState =
          std::to_string(ringIndex) + "-" + std::to_string(keyIndex);
      if (seen.count(currentState)) {
        continue;
      }

      seen.insert(currentState);
      for (int nextIndex : indexMap[key[keyIndex]]) {
        heap.push({totalSteps + countSteps(ringIndex, nextIndex, ringLen),
                   nextIndex, keyIndex + 1});
      }
    }
    return totalSteps + keyLen;
  }

 private:
  int countSteps(int curr, int next, int ringLen) {
    const int stepsBetween = std::abs(curr - next);
    const int stepsAround = ringLen - stepsBetween;
    return std::min(stepsBetween, stepsAround);
  }
};

struct Input {
  std::string ring;
  std::string key;
};

int main() {
  const Input inputs[] = {{"godding", "gd"}, {"godding", "godding"}

  };

  for (auto input : inputs) {
    Solution s;
    const auto result = s.findRotateSteps(input.ring, input.key);

    std::cout << result << std::endl;
  }

  return 0;
}
