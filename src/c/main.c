#include "./main.h"

void helper(struct TreeNode *node, unsigned int *first_min,
	    unsigned int *second_min)
{
	if (node == NULL) {
		return;
	}

	const unsigned int val = node->val;

	if (val < *first_min) {
		*second_min = *first_min;
		*first_min = val;
	} else if ((val > *first_min) && (val < *second_min)) {
		*second_min = val;
	}

	helper(node->left, first_min, second_min);
	helper(node->right, first_min, second_min);
}

int findSecondMinimumValue(struct TreeNode *root)
{
	if (root == NULL) {
		return -1;
	}

	const unsigned int MAX = 0xFFFFFFFF;

	unsigned int first_min = root->val;
	unsigned int second_min = MAX;
	helper(root, &first_min, &second_min);

	return second_min == MAX ? -1 : second_min;
}
