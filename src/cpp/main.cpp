#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
 public:
  int maximalRectangle(std::vector<std::vector<char>>& matrix) {
    const int colCount = matrix[0].size();
    std::vector<int> heights(colCount + 1, 0);
    int result = 0;

    for (const std::vector<char>& row : matrix) {
      for (int col = 0; col < colCount; col += 1) {
        if (row[col] == '1') {
          heights[col] += 1;
        } else {
          heights[col] = 0;
        }
      }

      std::vector<int> stack;
      stack.reserve(colCount + 1);
      for (int col = 0; col <= colCount; col += 1) {
        while (!stack.empty() &&
               heights.at(col) < heights.at(*stack.rbegin())) {
          const int h = heights.at(*stack.rbegin());
          stack.pop_back();
          const int w = stack.empty() ? col : col - *stack.rbegin() - 1;
          result = std::max(result, w * h);
        }
        stack.push_back(col);
      }
    }

    return result;
  }
};

struct Input {
  std::vector<std::vector<char>> matrix;
};

int main() {
  const Input inputs[] = {{{{{'1', '0', '1', '0', '0'},
                             {'1', '0', '1', '1', '1'},
                             {'1', '1', '1', '1', '1'},
                             {'1', '0', '0', '1', '0'}}}},
                          {{{'0'}}},
                          {{{'1'}}}

  };

  for (auto input : inputs) {
    Solution s;
    const auto result = s.maximalRectangle(input.matrix);
    std::cout << result << std::endl;
  }

  return 0;
}
