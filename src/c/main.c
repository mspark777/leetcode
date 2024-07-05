#include "./main.h"

int *nodesBetweenCriticalPoints(struct ListNode *head, int *return_size)
{
	*return_size = 2;
	int *result = malloc(*return_size * sizeof(int));
	result[0] = -1;
	result[1] = -1;

	if (head == NULL) {
		return result;
	}

	const int MAX = 0x7FFFFFFF;
	int min_distance = MAX;
	struct ListNode *prev_node = head;
	struct ListNode *curr_node = head->next;
	int curr = 1;
	int prev_critical = 0;
	int first_critical = 0;

	while (curr_node->next != NULL) {
		const int curr_val = curr_node->val;
		const int next_val = curr_node->next->val;
		const int prev_val = prev_node->val;
		const int is_minima = (curr_val < prev_val) &&
				      (curr_val < next_val);
		const int is_maxima = (curr_val > prev_val) &&
				      (curr_val > next_val);
		const int is_critical = is_minima | is_maxima;
		if (is_critical) {
			if (prev_critical) {
				const int diff = curr - prev_critical;
				if (diff < min_distance) {
					min_distance = diff;
				}
				prev_critical = curr;
			} else {
				prev_critical = curr;
				first_critical = curr;
			}
		}

		curr += 1;
		prev_node = curr_node;
		curr_node = curr_node->next;
	}

	if (min_distance != MAX) {
		result[0] = min_distance;
		result[1] = prev_critical - first_critical;
	}

	return result;
}
