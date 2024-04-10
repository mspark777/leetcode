#include <algorithm>
#include <iostream>
#include <iterator>
#include <vector>

class Solution {
 public:
  std::vector<int> deckRevealedIncreasing(std::vector<int>& deck) {
    const int deckSize = deck.size();
    std::vector<int> result(deckSize);
    bool skip = false;
    int deckIdx = 0;
    int resIdx = 0;

    std::sort(deck.begin(), deck.end());

    while (deckIdx < deckSize) {
      if (result[resIdx] == 0) {
        if (!skip) {
          result[resIdx] = deck[deckIdx];
          deckIdx += 1;
        }

        skip = !skip;
      }

      resIdx = (resIdx + 1) % deckSize;
    }

    return result;
  }
};

struct Input {
  std::vector<int> deck;
};

int main() {
  const Input inputs[] = {{{17, 13, 11, 2, 3, 5, 7}}, {{1, 1000}}};

  for (auto input : inputs) {
    Solution s;
    const std::vector<int> result = s.deckRevealedIncreasing(input.deck);
    std::copy(result.begin(), result.end(),
              std::ostream_iterator<int>(std::cout, " "));
    std::cout << std::endl;
  }

  return 0;
}
