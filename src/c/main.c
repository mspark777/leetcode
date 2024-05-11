#include "./main.h"
#include <stdio.h>
#include <stdlib.h>
#include <float.h>

#define static_cast(t, v) ((t)v)

struct wage_to_quality_ratio {
	double ratio;
	int quality;
};

int wage_to_quality_ratio_sort_partition(struct wage_to_quality_ratio *arr,
					 int left, int right)
{
	int pivot_idx = -1;
	for (int i = left; i < right; i += 1) {
		if (arr[i].ratio > arr[i + 1].ratio) {
			pivot_idx = i;
			break;
		}
	}

	if (pivot_idx < 0) {
		return pivot_idx;
	}

	const double pivot_ratio = arr[pivot_idx].ratio;
	arr[pivot_idx] = arr[right];
	arr[right] = arr[pivot_idx];

	int l = left - 1;
	for (int r = left; r < right; r += 1) {
		const double r_ratio = arr[r].ratio;
		if (r_ratio < pivot_ratio) {
			l += 1;
			arr[r] = arr[l];
			arr[l] = arr[r];
		}
	}

	const int next_pivot_idx = l + 1;
	arr[right] = arr[next_pivot_idx];
	arr[next_pivot_idx] = arr[pivot_idx];
	return next_pivot_idx;
}

void wage_to_quality_sort(struct wage_to_quality_ratio *arr, int left,
			  int right)
{
	if (left < right) {
		const int pivot =
			wage_to_quality_ratio_sort_partition(arr, left, right);
		if (pivot >= 0) {
			wage_to_quality_sort(arr, left, pivot - 1);
			wage_to_quality_sort(arr, pivot + 1, right);
		}
	}
}

double mincostToHireWorkers(int *quality_list, int quality_size, int *wage_list,
			    int wage_size, int k)
{
	const int n = quality_size;
	double result = DBL_MAX;
	struct wage_to_quality_ratio *wage_to_quality_ratio_list =
		malloc(sizeof(struct wage_to_quality_ratio));
	for (int i = 0; i < n; i += 1) {
		const int quality = quality_list[i];
		const double ratio = static_cast(double, wage_list[i]) /
				     static_cast(double, quality);
		wage_to_quality_ratio_list[i].ratio = ratio;
		wage_to_quality_ratio_list[i].quality = quality;
	}
	wage_to_quality_sort(wage_to_quality_ratio_list, 0, n);

	double current_total_quality = 0.0;

	free(wage_to_quality_ratio_list);
	wage_to_quality_ratio_list = NULL;
	return result;
}

/*
 double mincostToHireWorkers(vector<int>& quality, vector<int>& wage, int k) {
        sort(wageToQualityRatio.begin(), wageToQualityRatio.end());

        // Use a priority queue to keep track of the highest quality workers
        priority_queue<int> highestQualityWorkers;

        // Iterate through workers
        for (int i = 0; i < n; i++) {
            highestQualityWorkers.push(wageToQualityRatio[i].second);
            currentTotalQuality += wageToQualityRatio[i].second;

            // If we have more than k workers,
            // remove the one with the highest quality
            if (highestQualityWorkers.size() > k) {
                currentTotalQuality -= highestQualityWorkers.top();
                highestQualityWorkers.pop();
            }

            // If we have exactly k workers,
            // calculate the total cost and update if it's the minimum
            if (highestQualityWorkers.size() == k) {
                totalCost = min(totalCost, currentTotalQuality *
                                               wageToQualityRatio[i].first);
            }
        }
        return totalCost;
    }
 */

int main()
{
	int quality0[] = { 10, 20, 5 };
	int wage0[] = { 70, 50, 30 };
	int size0 = sizeof(quality0) / sizeof(quality0[0]);
	int k0 = 2;
	double result0 =
		mincostToHireWorkers(quality0, size0, wage0, size0, k0);
	printf("%f\n", result0);

	int quality1[] = { 3, 1, 10, 10, 1 };
	int wage1[] = { 4, 8, 2, 2, 7 };
	int size1 = sizeof(wage1) / sizeof(wage1[0]);
	int k1 = 3;
	double result1 =
		mincostToHireWorkers(quality1, size1, wage1, size1, k1);
	printf("%f\n", result1);
}

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
