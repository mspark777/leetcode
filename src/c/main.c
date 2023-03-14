#include "./lib.h"
#include <stdio.h>
#include <stdlib.h>

#define TreeNode tree_node

int travel(struct TreeNode *node, long long sum) {
  if (node == NULL) {
    return 0;
  }

  const int newsum = (sum * 10) + node->val;
  struct TreeNode *left = node->left;
  struct TreeNode *right = node->right;

  if ((left == NULL) && (right == NULL)) {
    return newsum;
  }

  return travel(left, newsum) + travel(right, newsum);
}

int sumNumbers(struct TreeNode *root) { return travel(root, 0); }

int main() {
  struct tree_node *inputs[] = {
      new_tree_node(1, new_tree_val(2), new_tree_val(3)),
      new_tree_node(4, new_tree_node(9, new_tree_val(5), new_tree_val(1)),
                    new_tree_val(0))};

  for (unsigned long i = 0; i < sizeof(inputs) / sizeof(inputs[0]); i += 1) {
    struct tree_node *root = inputs[i];
    const int result = sumNumbers(root);
    free_tree_node(root);
    printf("%d\n", result);
  }
  return 0;
}
