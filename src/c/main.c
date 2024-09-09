#include "./main.h"

/**
 * Return an array of arrays of size *returnSize.
 * The sizes of the arrays are returned as *returnColumnSizes array.
 * Note: Both returned array and *columnSizes array must be malloced, assume caller calls free().
 */
int **spiralMatrix(int m, int n, struct ListNode *head, int *return_size,
		   int **return_column_sizes)
{
	int **result = malloc(sizeof(int *) * m);
	*return_size = m;
	*return_column_sizes = malloc(sizeof(int) * m);
	for (int i = 0; i < m; i += 1) {
		result[i] = malloc(sizeof(int) * n);
		(*return_column_sizes)[i] = n;

		for (int j = 0; j < n; j += 1) {
			result[i][j] = -1;
		}
	}

	int directions[][2] = { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };

	int r = 0;
	int c = 0;
	int dir = 0;
	struct ListNode *node = head;
	while (node != NULL) {
		result[r][c] = node->val;

		int newr = r + directions[dir][0];
		int newc = c + directions[dir][1];

		int roate = newr < 0 || newc < 0 || newr >= m || newc >= n ||
			    result[newr][newc] != -1;
		if (roate) {
			dir = (dir + 1) % 4;
		}

		r += directions[dir][0];
		c += directions[dir][1];

		node = node->next;
	}

	return result;
}
