#include "./main.h"
#include <stdio.h>
#include <stdlib.h>

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

int sort_partition(int *arr, int left, int right)
{
	int pivot_idx = -1;
	for (int i = left; i < right; i += 1) {
		if (arr[i] < arr[i + 1]) {
			pivot_idx = i;
			break;
		}
	}

	if (pivot_idx < 0) {
		return pivot_idx;
	}

	const int pivot = arr[pivot_idx];
	arr[pivot_idx] = arr[right];
	arr[right] = pivot;

	int l = left - 1;
	for (int r = left; r < right; r += 1) {
		const int n = arr[r];
		if (n > pivot) {
			l += 1;
			arr[r] = arr[l];
			arr[l] = n;
		}
	}

	const int next_pivot_idx = l + 1;
	arr[right] = arr[next_pivot_idx];
	arr[next_pivot_idx] = pivot;
	return next_pivot_idx;
}

void sort(int *arr, int left, int right)
{
	if (left < right) {
		const int pivot = sort_partition(arr, left, right);
		if (pivot >= 0) {
			sort(arr, left, pivot - 1);
			sort(arr, pivot + 1, right);
		}
	}
}
