#include "./main.h"
#include <iostream>
#include <iterator>
#include <vector>

class Solution {
public:
  std::vector<int> inorderTraversal(TreeNode *root) {
    std::vector<int> result;

    this->traversal(root, result);
    return result;
  }

  void traversal(TreeNode *node, std::vector<int> &result) {
    if (node != nullptr) {
      this->traversal(node->left, result);
      result.push_back(node->val);
      this->traversal(node->right, result);
    }
  }
};

int main() {
  TreeNode *input[] = {new_tree_right(1, new_tree_left(2, new_tree_val(3))),
                       nullptr, new_tree_val(1)};
  const int count = (int)(sizeof(input) / sizeof(TreeNode *));

  for (int i = 0; i < count; i += 1) {
    auto root = input[i];
    auto result = Solution().inorderTraversal(root);
    std::copy(result.begin(), result.end(),
              std::ostream_iterator<int>(std::cout, " "));
    std::cout << std::endl;

    free_tree_node(root);
  }

  return 0;
}
