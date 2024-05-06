#ifndef __MAIN_H__
#define __MAIN_H__

struct ListNode {
	int val;
	struct ListNode *next;
};

struct ListNode *list_node_create(const int *nums, const int num_count);
struct ListNode *list_node_delete(struct ListNode *head);
void list_node_print(const struct ListNode *head);
struct ListNode *list_node_reverse(struct ListNode *head);

#endif
