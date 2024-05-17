#include <stdio.h>
#include "./main.h"

struct TreeNode *removeLeafNodes(struct TreeNode *node, int target)
{
	if (node == NULL) {
		return NULL;
	}

	struct TreeNode *left = removeLeafNodes(node->left, target);
	struct TreeNode *right = removeLeafNodes(node->right, target);

	node->left = left;
	node->right = right;

	const int is_leaf = (left == NULL) && (right == NULL);
	if (is_leaf) {
		return node->val == target ? NULL : node;
	}

	return node;
}
