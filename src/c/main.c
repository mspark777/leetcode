#include "./lib.h"
#include <stdio.h>
#include <stdlib.h>

#define TreeNode tree_node

enum boolean { true = 1, false = 0 };

int count_nodes(struct TreeNode *node) {
  if (node == NULL) {
    return 0;
  }

  return 1 + count_nodes(node->left) + count_nodes(node->right);
}

int dfs(struct TreeNode *node, int index, int n) {
  if (node == NULL) {
    return true;
  }

  if (index >= n) {
    return false;
  }

  const int newindex = 2 * index + 1;
  return dfs(node->left, newindex, n) && dfs(node->right, newindex + 1, n);
}

int isCompleteTree(struct TreeNode *root) {
  return dfs(root, 0, count_nodes(root));
}

int main() {
  struct tree_node *inputs[] = {
      new_tree_node(1, new_tree_node(2, new_tree_val(4), new_tree_val(5)),
                    new_tree_left(3, new_tree_val(6))),
      new_tree_node(1, new_tree_node(2, new_tree_val(4), new_tree_val(5)),
                    new_tree_right(3, new_tree_val(7)))};

  for (unsigned long i = 0; i < sizeof(inputs) / sizeof(inputs[0]); i += 1) {
    struct tree_node *root = inputs[i];
    const int result = isCompleteTree(root);
    free_tree_node(root);
    printf("%d\n", result);
  }
  return 0;
}
