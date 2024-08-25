#include "./main.h"

void get_tree_node_count(struct TreeNode *node, int *count)
{
	if (node != NULL) {
		*count += 1;
		get_tree_node_count(node->left, count);
		get_tree_node_count(node->right, count);
	}
}

void post(struct TreeNode *node, int *nums, int *pos)
{
	if (node != NULL) {
		post(node->left, nums, pos);
		post(node->right, nums, pos);
		const int p = *pos;
		nums[p] = node->val;
		*pos += 1;
	}
}

int *postorderTraversal(struct TreeNode *root, int *return_size)
{
	*return_size = 0;
	get_tree_node_count(root, return_size);
	int *nums = malloc(sizeof(int) * (*return_size));

	int pos = 0;
	post(root, nums, &pos);

	return nums;
}
