#include "./main.h"

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
struct TreeNode *mergeTrees(struct TreeNode *root1, struct TreeNode *root2)
{
	if (root1 == NULL) {
		return root2;
	} else if (root2 == NULL) {
		return root1;
	}

	root1->val += root2->val;
	if (root1->left == NULL) {
		root1->left = root2->left;
	} else {
		mergeTrees(root1->left, root2->left);
	}

	if (root1->right == NULL) {
		root1->right = root2->right;
	} else {
		mergeTrees(root1->right, root2->right);
	}

	return root1;
}
