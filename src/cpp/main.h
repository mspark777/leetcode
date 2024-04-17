#ifndef __LEETCODE_CPP_MAIN_H__
#define __LEETCODE_CPP_MAIN_H__

#include <string>

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

void cleanTreeNode(TreeNode *root);
int parseLeetCodeBinaryTree(std::string &rawString, TreeNode **pRoot);
#endif
