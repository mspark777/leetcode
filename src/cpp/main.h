#ifndef __MAIN_H__
#define __MAIN_H__

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};
TreeNode *new_tree_node(int val, TreeNode *left, TreeNode *right);
TreeNode *new_tree_left(int val, TreeNode *left);
TreeNode *new_tree_right(int val, TreeNode *right);
TreeNode *new_tree_val(int val);
void free_tree_node(TreeNode *root);

#endif
