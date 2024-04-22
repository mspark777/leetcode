#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <vector>

class Solution {
 public:
  int openLock(std::vector<std::string>& deadends, std::string target) {
    const std::map<char, char> nextSlot = {
        {'0', '1'}, {'1', '2'}, {'2', '3'}, {'3', '4'}, {'4', '5'},
        {'5', '6'}, {'6', '7'}, {'7', '8'}, {'8', '9'}, {'9', '0'}};

    const std::map<char, char> prevSlot = {
        {'0', '9'}, {'1', '0'}, {'2', '1'}, {'3', '2'}, {'4', '3'},
        {'5', '4'}, {'6', '5'}, {'7', '6'}, {'8', '7'}, {'9', '8'}};

    std::set<std::string> visited(deadends.begin(), deadends.end());
    std::queue<std::string> queue;

    if (visited.find("0000") != visited.end()) {
      return -1;
    }

    int result = 0;
    queue.push("0000");
    visited.insert("0000");

    while (!queue.empty()) {
      const int nodeCount = static_cast<int>(queue.size());
      for (int i = 0; i < nodeCount; i += 1) {
        const std::string curr = queue.front();
        queue.pop();

        if (curr == target) {
          return result;
        }

        for (int wheel = 0; wheel < 4; wheel += 1) {
          std::string prev = curr;
          prev[wheel] = nextSlot.at(curr.at(wheel));
          if (visited.find(prev) == visited.end()) {
            queue.push(prev);
            visited.insert(prev);
          }

          std::string next = curr;
          next[wheel] = prevSlot.at(curr.at(wheel));
          if (visited.find(next) == visited.end()) {
            queue.push(next);
            visited.insert(next);
          }
        }
      }

      result += 1;
    }

    return -1;
  };
};

struct Input {
  std::vector<std::string> deadends;
  std::string target;
};

int main() {
  const Input inputs[] = {
      {{"0201", "0101", "0102", "1212", "2002"}, "0202"},
      {{"8888"}, "0009"},
      {{"8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"},
       "8888"}};

  for (auto input : inputs) {
    Solution s;
    const auto result = s.openLock(input.deadends, input.target);

    std::cout << result << std::endl;
  }

  return 0;
}
