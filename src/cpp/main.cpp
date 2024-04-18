#include <iostream>
#include <vector>

enum Cell {
  WATER = 0,
  ISLAND = 1,
};

class Solution {
 public:
  int islandPerimeter(std::vector<std::vector<int>>& grid) {
    const int rowCount = grid.size();
    const int colCount = grid.at(0).size();
    const int lastRow = rowCount - 1;
    const int lastCol = colCount - 1;
    int result = 0;

    for (int row = 0; row < rowCount; row += 1) {
      for (int col = 0; col < colCount; col += 1) {
        if (grid[row][col] != Cell::ISLAND) {
          continue;
        }

        result += 4;
        if ((row > 0) && (grid[row - 1][col] == Cell::ISLAND)) {
          result -= 1;
        }

        if ((col > 0) && (grid[row][col - 1] == Cell::ISLAND)) {
          result -= 1;
        }

        if ((row < lastRow) && (grid[row + 1][col] == Cell::ISLAND)) {
          result -= 1;
        }

        if ((col < lastCol) && (grid[row][col + 1] == Cell::ISLAND)) {
          result -= 1;
        }
      }
    }

    return result;
  }
};

struct Input {
  std::vector<std::vector<int>> grid;
};

int main() {
  const Input inputs[] = {
      {{{0, 1, 0, 0}, {1, 1, 1, 0}, {0, 1, 0, 0}, {1, 1, 0, 0}}},
      {{{1}}},
      {{{1, 0}}}};

  for (auto input : inputs) {
    Solution s;
    const auto result = s.islandPerimeter(input.grid);
    std::cout << result << std::endl;
  }

  return 0;
}
