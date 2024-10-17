#include "./main.h"

struct TreeNode *searchBST(struct TreeNode *root, int val)
{
	if (root == NULL) {
		return NULL;
	}

	if (root->val == val) {
		return root;
	}

	struct TreeNode *left = searchBST(root->left, val);
	return left != NULL ? left : searchBST(root->right, val);
}
