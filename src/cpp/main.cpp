#include <iostream>
#include <vector>

class Solution {
 public:
  int minFallingPathSum(std::vector<std::vector<int>>& grid) {
    const int N = grid.size();
    int nextMin1C = -1;
    int nextMin1 = -1;
    int nextMin2 = -1;

    for (int col = 0; col < N; col += 1) {
      const int c = grid[N - 1][col];
      if (nextMin1 == -1 || c <= nextMin1) {
        nextMin2 = nextMin1;
        nextMin1 = c;
        nextMin1C = col;
      } else if (nextMin2 == -1 || c <= nextMin2) {
        nextMin2 = c;
      }
    }

    for (int row = N - 2; row >= 0; row -= 1) {
      int min1C = -1;
      int min1 = -1;
      int min2 = -1;
      for (int col = 0; col < N; col += 1) {
        int value = grid[row][col] + (col != nextMin1C ? nextMin1 : nextMin2);
        if (min1 == -1 || value < min1) {
          min2 = min1;
          min1 = value;
          min1C = col;
        } else if (min2 == -1 || value < min2) {
          min2 = value;
        }
      }

      nextMin1C = min1C;
      nextMin1 = min1;
      nextMin2 = min2;
    }

    return nextMin1;
  }
};

struct Input {
  std::vector<std::vector<int>> grid;
};

int main() {
  const Input inputs[] = {{{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}}, {{{7}}}

  };

  for (auto input : inputs) {
    Solution s;
    const auto result = s.minFallingPathSum(input.grid);

    std::cout << result << std::endl;
  }

  return 0;
}
