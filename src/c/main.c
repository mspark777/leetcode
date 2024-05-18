#include <stdio.h>
#include "./main.h"

int abs(int i)
{
	return i >= 0 ? i : -1;
}

int dfs(struct TreeNode *node, int *move)
{
	if (node == NULL) {
		return 0;
	}

	const int left = dfs(node->left, move);
	const int right = dfs(node->right, move);

	*move += abs(left) + abs(right);

	return (node->val - 1) + left + right;
}

int distributeCoins(struct TreeNode *root)
{
	int result = 0;
	dfs(root, &result);
	return result;
}
