#include <iostream>
#include <set>
#include <stack>
#include <utility>
#include <vector>

enum Cell {
  WATER = '0',
  ISLAND = '1',
};

class Solution {
 public:
  int numIslands(const std::vector<std::vector<char>>& grid) {
    const int rowCount = static_cast<int>(grid.size());
    const int colCount = static_cast<int>(grid.at(0).size());
    std::set<std::pair<int, int>> visits;

    int result = 0;
    for (int row = 0; row < rowCount; row += 1) {
      for (int col = 0; col < colCount; col += 1) {
        if (grid[row][col] == Cell::WATER) {
          continue;
        } else if (visits.find({row, col}) != visits.end()) {
          continue;
        }

        result += 1;
        this->markVisit(grid, visits, row, col);
      }
    }

    return result;
  }

 private:
  void markVisit(const std::vector<std::vector<char>>& grid,
                 std::set<std::pair<int, int>>& visits, int row, int col) {
    const int rowCount = static_cast<int>(grid.size());
    const int colCount = static_cast<int>(grid.at(0).size());

    std::stack<std::pair<int, int>> stack;
    stack.push({row, col});

    while (!stack.empty()) {
      const std::pair<int, int> cur = stack.top();
      stack.pop();

      const int r = cur.first;
      const int c = cur.second;

      if ((r < 0) || (c < 0)) {
        continue;
      } else if ((r >= rowCount) || (c >= colCount)) {
        continue;
      } else if (grid[r][c] == Cell::WATER) {
        continue;
      } else if (visits.find(cur) != visits.end()) {
        continue;
      }

      visits.insert(cur);
      stack.push({r - 1, c});
      stack.push({r + 1, c});
      stack.push({r, c - 1});
      stack.push({r, c + 1});
    }
  }
};

struct Input {
  std::vector<std::vector<char>> grid;
};

int main() {
  const Input inputs[] = {{{{'1', '1', '1', '1', '0'},
                            {'1', '1', '0', '1', '0'},
                            {'1', '1', '0', '0', '0'},
                            {'0', '0', '0', '0', '0'}}},
                          {

                              {{'1', '1', '0', '0', '0'},
                               {'1', '1', '0', '0', '0'},
                               {'0', '0', '1', '0', '0'},
                               {'0', '0', '0', '1', '1'}}

                          }};

  for (auto input : inputs) {
    Solution s;
    const auto result = s.numIslands(input.grid);
    std::cout << result << std::endl;
  }

  return 0;
}
