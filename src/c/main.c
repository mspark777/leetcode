#include <stdio.h>
#include <stdlib.h>
#include "./main.h"

struct ListNode *doubleIt(struct ListNode *head)
{
	if (head->val > 4) {
		struct ListNode *temp = head;
		head = calloc(1, sizeof(*head));
		head->next = temp;
	}

	for (struct ListNode *node = head; node != NULL; node = node->next) {
		node->val = (node->val * 2) % 10;
		const struct ListNode *next = node->next;
		if ((next != NULL) && (next->val > 4)) {
			node->val += 1;
		}
	}

	return head;
}

int main()
{
	const int values0[] = { 1, 8, 9 };
	struct ListNode *head0 =
		list_node_create(values0, sizeof(values0) / sizeof(values0[0]));
	list_node_print(doubleIt(head0));

	const int values1[] = { 9, 9, 9 };
	struct ListNode *head1 =
		list_node_create(values1, sizeof(values1) / sizeof(values1[0]));
	list_node_print(doubleIt(head1));

	const int values2[] = { 0 };
	struct ListNode *head2 =
		list_node_create(values2, sizeof(values2) / sizeof(values2[0]));
	list_node_print(doubleIt(head2));
}
