#include "./main.h"

int *alloc_list()
{
	return calloc(12, sizeof(int));
}

int *post_order(struct TreeNode *node, int distance)
{
	if (node == NULL) {
		return alloc_list();
	} else if (node->left == NULL && node->right == NULL) {
		int *leaf_node_count_list = alloc_list();
		leaf_node_count_list[0] = 1;
		return leaf_node_count_list;
	}

	int *left = post_order(node->left, distance);
	int *right = post_order(node->right, distance);
	int *current = alloc_list();

	for (int i = 0; i < 10; i += 1) {
		current[i + 1] = left[i] + right[i];
	}

	current[11] += left[11] + right[11];
	for (int d1 = 0; d1 <= distance; d1++) {
		for (int d2 = 0; d2 <= distance; d2++) {
			int d = d1 + d2 + 2;
			if (d <= distance) {
				current[11] += left[d1] * right[d2];
			}
		}
	}

	free(left);
	free(right);
	return current;
}

int countPairs(struct TreeNode *root, int distance)
{
	int *counts = post_order(root, distance);
	int result = counts[11];
	free(counts);

	return result;
}
