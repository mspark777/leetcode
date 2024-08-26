#include "./main.h"

void get_node_count(struct Node *node, int *count)
{
	if (node == NULL) {
		return;
	}

	const int children = node->numChildren;
	*count += children;
	for (int i = 0; i < children; i += 1) {
		get_node_count(node->children[i], count);
	}
}

void post(struct Node *node, int *nums, int *pos)
{
	if (node == NULL) {
		return;
	}

	const int children = node->numChildren;
	for (int i = 0; i < children; i += 1) {
		post(node->children[i], nums, pos);
	}

	const int p = *pos;
	nums[p] = node->val;
	*pos += 1;
}

int *postorder(struct Node *root, int *return_size)
{
	if (root == NULL) {
		*return_size = 0;
		return malloc(sizeof(int));
	}

	*return_size = 1;
	get_node_count(root, return_size);
	int *nums = malloc(sizeof(int) * (*return_size));

	int pos = 0;
	post(root, nums, &pos);

	return nums;
}
