#include "./main.h"

#include <iostream>
#include <vector>

class Solution {
 public:
  int sumNumbers(TreeNode* root) { return this->sum(root, 0); }

 private:
  int sum(TreeNode* node, int s) {
    if (node == nullptr) {
      return 0;
    }

    const int news = (s * 10) + node->val;
    if (node->left == nullptr && node->right == nullptr) {
      return news;
    }

    return this->sum(node->left, news) + this->sum(node->right, news);
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
