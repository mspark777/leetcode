#include "./main.h"

TreeNode *new_tree_node(int val, TreeNode *left, TreeNode *right) {
  auto node = new (TreeNode);
  node->val = val;
  node->left = left;
  node->right = right;

  return node;
}

TreeNode *new_tree_left(int val, TreeNode *left) {
  return new_tree_node(val, left, nullptr);
}

TreeNode *new_tree_right(int val, TreeNode *right) {
  return new_tree_node(val, nullptr, right);
}

TreeNode *new_tree_val(int val) { return new_tree_node(val, nullptr, nullptr); }

void free_tree_node(TreeNode *root) {
  if (root != nullptr) {
    free_tree_node(root->left);
    free_tree_node(root->right);
    delete root;
  }
}
