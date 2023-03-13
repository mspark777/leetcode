#include "./lib.h"
#include <stdio.h>
#include <stdlib.h>

#define TreeNode tree_node

int is_mirror(struct TreeNode *left, struct TreeNode *right) {
  if ((left == NULL) && (right == NULL)) {
    return 1;
  } else if ((left == NULL) || (right == NULL)) {
    return 0;
  }

  return (left->val == right->val) && is_mirror(left->left, right->right) &&
         is_mirror(left->right, right->left);
}

int isSymmetric(struct TreeNode *root) {
  if (root == NULL) {
    return 1;
  }

  return is_mirror(root->left, root->right);
}

void problem0(void) {
  struct TreeNode *root =
      new_tree_node(1, new_tree_node(2, new_tree_val(3), new_tree_val(4)),
                    new_tree_node(2, new_tree_val(4), new_tree_val(3)));

  const int result = isSymmetric(root);
  free_tree_node(root);

  printf("%d\n", result);
}

void problem1(void) {
  struct TreeNode *root = new_tree_node(1, new_tree_right(2, new_tree_val(3)),
                                        new_tree_right(2, new_tree_val(3)));

  const int result = isSymmetric(root);
  free_tree_node(root);

  printf("%d\n", result);
}

typedef void (*problem_t)(void);
int main() {
  const problem_t problems[] = {problem0, problem1};

  for (unsigned long i = 0; i < sizeof(problems) / sizeof(problems[0]);
       i += 1) {
    problems[i]();
  }
  return 0;
}
