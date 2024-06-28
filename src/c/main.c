#include <math.h>
#include <stdio.h>
#include "./main.h"

void rotate_right(struct TreeNode *parent, struct TreeNode *node)
{
	struct TreeNode *temp = node->left;
	node->left = temp->right;
	temp->right = node;
	parent->right = temp;
}

void rotate_left(struct TreeNode *parent, struct TreeNode *node)
{
	struct TreeNode *temp = node->right;
	node->right = temp->left;
	temp->left = node;
	parent->right = temp;
}

void rotate(struct TreeNode *node, int count)
{
	struct TreeNode *current = node;
	for (int i = 0; i < count; i += 1) {
		struct TreeNode *temp = current->right;
		rotate_left(current, temp);
		current = current->right;
	}
}

struct TreeNode *balanceBST(struct TreeNode *root)
{
	if (root == NULL) {
		return root;
	}

	struct TreeNode dummy = { 0, NULL, NULL };
	dummy.right = root;

	struct TreeNode *current = &dummy;
	while (current->right != NULL) {
		if (current->right->left) {
			rotate_right(current, current->right);
		} else {
			current = current->right;
		}
	}

	int node_count = 0;
	current = dummy.right;
	while (current != NULL) {
		node_count += 1;
		current = current->right;
	}

	int depth = ((int)(pow(2.0, floor(log2(node_count + 1))))) - 1;
	rotate(&dummy, node_count - depth);
	while (depth > 1) {
		depth /= 2;
		rotate(&dummy, depth);
	}

	return dummy.right;
}
