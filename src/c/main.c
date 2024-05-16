#include <stdio.h>
#include "./main.h"

const int or = 2;
const int and = 3;

int evaluateTree(struct TreeNode *node)
{
	const int value = node->val;
	struct TreeNode *left = node->left;
	struct TreeNode *right = node->right;
	if ((left == NULL) || (right == NULL)) {
		return value;
	}

	return value == or ? evaluateTree(left) || evaluateTree(right) :
			     evaluateTree(left) && evaluateTree(right);
}
