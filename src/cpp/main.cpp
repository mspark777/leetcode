#include <algorithm>
#include <iostream>
#include <iterator>
#include <set>
#include <stack>
#include <utility>
#include <vector>

enum Cell {
  FORESTED = 0,
  FARMLAND = 1,
};

class Solution {
 public:
  std::vector<std::vector<int>> findFarmland(
      std::vector<std::vector<int>>& land) {
    std::set<std::pair<int, int>> visit;

    std::vector<std::vector<int>> result;

    for (int row = 0; row < static_cast<int>(land.size()); row += 1) {
      for (int col = 0; col < static_cast<int>(land.at(row).size()); col += 1) {
        if (land[row][col] == Cell::FORESTED) {
          continue;
        } else if (visit.find({row, col}) != visit.end()) {
          continue;
        } else {
          const std::vector<int> group = this->travel(land, visit, row, col);
          result.push_back(group);
        }
      }
    }

    return result;
  }

 private:
  std::vector<int> travel(const std::vector<std::vector<int>>& land,
                          std::set<std::pair<int, int>>& visit, int row,
                          int col) {
    const int lastRow = static_cast<int>(land.size()) - 1;
    const int lastCol = static_cast<int>(land.at(0).size()) - 1;

    std::stack<std::pair<int, int>> stack;

    stack.push({row, col});

    int left = lastCol;
    int right = 0;
    int top = lastRow;
    int bottom = 0;

    while (!stack.empty()) {
      const std::pair<int, int> cur = stack.top();
      stack.pop();

      const int r = cur.first;
      const int c = cur.second;

      if ((r < 0) || (c < 0)) {
        continue;
      } else if ((r > lastRow) || (c > lastCol)) {
        continue;
      } else if (visit.find(cur) != visit.end()) {
        continue;
      } else if (land[r][c] == Cell::FORESTED) {
        continue;
      }

      if ((c < left) || (r < top)) {
        left = c;
        top = r;
      }

      if ((c > right) || (r > bottom)) {
        right = c;
        bottom = r;
      }

      visit.insert(cur);
      stack.push({r - 1, c});
      stack.push({r + 1, c});
      stack.push({r, c - 1});
      stack.push({r, c + 1});
    }

    return {top, left, bottom, right};
  }
};

struct Input {
  std::vector<std::vector<int>> land;
};

int main() {
  const Input inputs[] = {
      {
          {{1, 0, 0}, {0, 1, 1}, {0, 1, 1}},
      },
      {{{1, 1}, {1, 1}}},
      {{{0}}},
      {{{1, 1}, {0, 0}}},
  };

  for (auto input : inputs) {
    Solution s;
    const auto result = s.findFarmland(input.land);

    for (auto row : result) {
      std::copy(row.begin(), row.end(),
                std::ostream_iterator<int>(std::cout, " "));
      std::cout << std::endl;
    }
    std::cout << std::endl;
  }

  return 0;
}
