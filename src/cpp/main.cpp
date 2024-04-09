#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
 public:
  int timeRequiredToBuy(std::vector<int>& tickets, int k) {
    int result = 0;
    for (int i = 0; i < static_cast<int>(tickets.size()); i += 1) {
      if (i <= k) {
        result += std::min(tickets[k], tickets[i]);
      } else {
        result += std::min(tickets[k] - 1, tickets[i]);
      }
    }

    return result;
  }
};

struct Input {
  std::vector<int> tickets;
  int k;
};

int main() {
  const Input inputs[] = {
      {{2, 3, 2}, 2}, {{5, 1, 1, 1}, 0}, {{84, 49, 5, 24, 70, 77, 87, 8}, 3}};

  for (auto input : inputs) {
    Solution s;
    std::cout << s.timeRequiredToBuy(input.tickets, input.k) << std::endl;
  }
  return 0;
}
