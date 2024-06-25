#include <stdio.h>
#include "./main.h"

struct TreeNode *get_successor(struct TreeNode *node)
{
	struct TreeNode *successor = node->right;
	while (successor->left != NULL && successor->left != node) {
		successor = successor->left;
	}
	return successor;
}

struct TreeNode *bstToGst(struct TreeNode *root)
{
	int sum = 0;
	struct TreeNode *node = root;

	while (node != NULL) {
		if (node->right == NULL) {
			sum += node->val;
			node->val = sum;
			node = node->left;
			continue;
		}

		struct TreeNode *successor = get_successor(node);
		if (successor->left == NULL) {
			successor->left = node;
			node = node->right;
		} else {
			successor->left = NULL;
			sum += node->val;
			node->val = sum;
			node = node->left;
		}
	}
	return root;
}
