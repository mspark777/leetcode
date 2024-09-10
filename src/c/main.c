#include "./main.h"

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

int calculate_gcd(int a, int b)
{
	while (b != 0) {
		int temp = b;
		b = a % b;
		a = temp;
	}

	return a;
}
struct ListNode *insertGreatestCommonDivisors(struct ListNode *head)
{
	if (head == NULL) {
		return head;
	}

	struct ListNode *node1 = head;
	struct ListNode *node2 = head->next;

	while (node2 != NULL) {
		struct ListNode *node = malloc(sizeof(struct ListNode));
		node->val = calculate_gcd(node1->val, node2->val);

		node1->next = node;
		node->next = node2;

		node1 = node2;
		node2 = node2->next;
	}

	return head;
}
