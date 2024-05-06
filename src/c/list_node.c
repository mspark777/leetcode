#include <stdio.h>
#include <stdlib.h>

struct ListNode {
	int val;
	struct ListNode *next;
};

struct ListNode *list_node_create(const int *nums, const int num_count)
{
	struct ListNode *head = calloc(num_count, sizeof(struct ListNode));

	for (int i = 0; i < num_count; i += 1) {
		head[i].val = nums[i];
		if ((i + 1) < num_count) {
			head[i].next = head + i + 1;
		}
	}

	return head;
}

struct ListNode *list_node_delete(struct ListNode *head)
{
	free(head);
	return NULL;
}

void list_node_print(const struct ListNode *head)
{
	for (const struct ListNode *node = head; node != NULL;
	     node = node->next) {
		printf("%d ", node->val);
	}
	printf("\n");
}

struct ListNode *list_node_reverse(struct ListNode *head)
{
	struct ListNode *prev = NULL;
	struct ListNode *current = head;

	while (current != NULL) {
		struct ListNode *next = current->next;
		current->next = prev;
		prev = current;
		current = next;
	}

	return prev;
}
