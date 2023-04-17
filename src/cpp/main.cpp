#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

class Solution {
public:
  std::vector<bool> kidsWithCandies(std::vector<int> &candies,
                                    int extra_candies) {
    int max_candy = -1;
    for (auto candy : candies) {
      max_candy = std::max(candy, max_candy);
    }
    max_candy -= extra_candies;

    std::vector<bool> result;
    result.reserve(candies.size());
    for (auto candy : candies) {
      result.push_back(candy >= max_candy);
    }

    return result;
  }
};

struct Input {
  std::vector<int> candies;
  const int extra_candies;
};

int main() {
  const Input inputs[] = {
      {.candies = std::vector<int>({2, 3, 5, 1, 3}), .extra_candies = 3},
      {.candies = std::vector<int>({4, 2, 1, 1, 2}), .extra_candies = 1},
      {.candies = std::vector<int>({12, 1, 12}), .extra_candies = 10},
  };

  for (auto input : inputs) {
    Solution solution;
    const auto result =
        solution.kidsWithCandies(input.candies, input.extra_candies);
    for (auto r : result) {
      std::cout << r << ", ";
    }
    std::cout << std::endl;
  }
}
