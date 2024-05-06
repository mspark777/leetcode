#include <stdio.h>
#include "./main.h"

struct ListNode *reverse(struct ListNode *head)
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

struct ListNode *removeNodes(struct ListNode *head)
{
	head = reverse(head);

	int max = 0;
	struct ListNode *prev = NULL;
	struct ListNode *current = head;
	while (current != NULL) {
		if (current->val < max) {
			prev->next = current->next;
			current = current->next;
		} else {
			max = current->val;
			prev = current;
			current = current->next;
		}
	}

	return reverse(head);
}

int main()
{
	const int values0[] = { 5, 2, 13, 3, 8 };
	struct ListNode *head0 =
		list_node_create(values0, sizeof(values0) / sizeof(values0[0]));
	list_node_print(removeNodes(head0));
	head0 = list_node_delete(head0);

	const int values1[] = { 1, 1, 1, 1 };
	struct ListNode *head1 =
		list_node_create(values1, sizeof(values1) / sizeof(values1[0]));
	list_node_print(removeNodes(head1));
	head1 = list_node_delete(head1);
}
