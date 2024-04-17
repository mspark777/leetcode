#include "./main.h"

#include <algorithm>
#include <iostream>
#include <queue>
#include <string>
#include <utility>

class Solution {
 public:
  std::string smallestFromLeaf(TreeNode* root) {
    if (root == nullptr) {
      return std::string();
    }

    std::string result;
    std::queue<std::pair<TreeNode*, std::string>> queue;

    queue.push({root, std::string(1, root->val + 'a')});

    while (!queue.empty()) {
      std::pair<TreeNode*, std::string> node = queue.front();
      queue.pop();

      if (node.first->left == nullptr && node.first->right == nullptr) {
        if (result.empty()) {
          result = node.second;
        } else {
          result = std::min(result, node.second);
        }
        continue;
      }

      if (node.first->left != nullptr) {
        queue.push(
            {node.first->left,
             (static_cast<char>(node.first->left->val + 'a')) + node.second});
      }

      if (node.first->right != nullptr) {
        queue.push(
            {node.first->right,
             (static_cast<char>(node.first->right->val + 'a')) + node.second});
      }
    }

    return result;
  }
};

struct Input {
  std::string root;
};

int main() {
  const Input inputs[] = {
      {"[0,1,2,3,4,3,4]"},
      {"[25,1,3,1,3,0,2]"},
      {"[2,2,1,null,1,0,null,0]"},
  };

  for (auto input : inputs) {
    Solution s;
    TreeNode* root = nullptr;
    parseLeetCodeBinaryTree(input.root, &root);
    const auto result = s.smallestFromLeaf(root);
    std::cout << result << std::endl;
    cleanTreeNode(root);
  }

  return 0;
}
