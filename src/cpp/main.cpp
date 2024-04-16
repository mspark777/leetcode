#include "./main.h"

#include <iostream>
#include <vector>

class Solution {
 public:
  TreeNode* addOneRow(TreeNode* root, int val, int depth) {
    if (depth < 2) {
      return new TreeNode(val, root, nullptr);
    }

    this->travel(root, nullptr, val, depth, true);
    return root;
  }

 private:
  void travel(TreeNode* node, TreeNode* parent, int val, int depth,
              bool leftSide) {
    if (node == nullptr && depth > 1 && parent == nullptr) {
      return;
    }

    if (depth == 1) {
      TreeNode* root = new TreeNode(val);
      if (leftSide) {
        root->left = node;
        if (parent != nullptr) {
          parent->left = root;
        }
      } else {
        root->right = node;
        if (parent != nullptr) {
          parent->right = root;
        }
      }
    } else if (node != nullptr) {
      this->travel(node->left, node, val, depth - 1, true);
      this->travel(node->right, node, val, depth - 1, false);
    }
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
